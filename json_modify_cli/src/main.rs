use clap_builder::Parser;
use serde_json::Value;
use std::io::Write;
use std::path::Path;
use std::{fs, io};
use tempfile::NamedTempFile;

mod args;

///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2025-11-03
///
fn main() {
    let args = args::Args::parse();
    //--
    if (args.debug) {
        println!("当前工作路径->{:?}", std::env::current_dir().unwrap());
    }

    // 要替换的新值，可以是任意 JSON 值
    let new_value = serde_json::json!(args.value);

    // 读取文件
    let text = fs::read_to_string(&args.input).unwrap();
    // 解析为动态 Value
    let mut json: Value = serde_json::from_str(&text)
        .map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("JSON parse error: {}", e),
            )
        })
        .unwrap();

    let new_input = new_value.as_str().unwrap_or_default();
    let output_path = args.output.or(Some(args.input)).unwrap();
    let path = &output_path;
    let dotted_path = args.key.as_ref();

    // 获取目标值的可变引用
    match get_value_mut_by_path(&mut json, dotted_path) {
        Some(target) => {
            // 尝试按目标原始类型进行转换
            match coerce_to_same_type(target, new_input) {
                Ok(new_val) => {
                    *target = new_val;
                    write_json_atomic(path, &json).unwrap();
                    println!("修改文件内容 {} : '{}' -> {}", path, dotted_path, new_input);
                }
                Err(e) => (),
            }
        }
        None => (),
    }

    /*let output_path = args.output.or(Some(args.input)).unwrap();
    // 修改指定 key 路径
    if set_json_value(&mut json, args.key.as_ref(), new_value) {
        // 原子性写回：写入临时文件然后重命名替换原文件
        write_json_atomic(&output_path, &json).unwrap();
        if (args.debug) {
            println!("Updated {} at path {}", output_path, args.key);
        }
    } else {
        if (args.debug) {
            println!("Path '{}' not found in JSON, no change made.", args.key);
        }
    }*/
}

/// 将用户输入字符串转换为与 existing 类型一致的 serde_json::Value
fn coerce_to_same_type(existing: &Value, input: &str) -> Result<Value, String> {
    match existing {
        Value::Null => {
            // 只有当用户显式输入 "null"（不区分大小写）时才转换为 Null
            if input.eq_ignore_ascii_case("null") {
                Ok(Value::Null)
            } else {
                // 也可以选择将其变为字符串或报错，这里报错更安全
                Err("Existing value is null: only 'null' input allowed to set Null".into())
            }
        }
        Value::Bool(_) => {
            let lowered = input.to_ascii_lowercase();
            match lowered.as_str() {
                "true" => Ok(Value::Bool(true)),
                "false" => Ok(Value::Bool(false)),
                _ => Err("Cannot parse input as bool (expected 'true' or 'false')".into()),
            }
        }
        Value::Number(_) => {
            // 尝试解析整数，再尝试浮点
            if let Ok(i) = input.parse::<i64>() {
                Ok(Value::Number(serde_json::Number::from(i)))
            } else if let Ok(f) = input.parse::<f64>() {
                // 注意： serde_json::Number::from_f64 返回 Option
                serde_json::Number::from_f64(f)
                    .map(Value::Number)
                    .ok_or_else(|| "Invalid float value (NaN/inf)".into())
            } else {
                Err("Cannot parse input as number (int or float)".into())
            }
        }
        Value::String(_) => {
            // 目标是字符串：直接保存原始输入
            Ok(Value::String(input.to_string()))
        }
        Value::Array(_) | Value::Object(_) => {
            // 对于数组/对象，期待输入是合法的 JSON 表示
            serde_json::from_str(input)
                .map_err(|e| format!("Failed to parse input as JSON array/object: {}", e))
        }
    }
}

/// 通过点路径找到可变引用到目标 Value
/// 如果路径不存在或在中间遇到非对象，则返回 None。
fn get_value_mut_by_path<'a>(root: &'a mut Value, dotted_path: &str) -> Option<&'a mut Value> {
    let parts: Vec<&str> = dotted_path.split('.').filter(|s| !s.is_empty()).collect();
    if parts.is_empty() {
        return None;
    }
    let mut cur = root;
    for key in parts {
        match cur {
            Value::Object(map) => {
                cur = map.get_mut(key)?;
            }
            _ => return None,
        }
    }
    Some(cur)
}

/// 原子性写回 JSON 文件（先写临时文件再重命名）
fn write_json_atomic<P: AsRef<Path>>(dest: P, json: &Value) -> io::Result<()> {
    let dest_path = dest.as_ref();
    let parent = dest_path.parent().unwrap_or_else(|| Path::new("."));
    let mut tmp = NamedTempFile::new_in(parent)?;
    // 保持漂亮格式
    let text = serde_json::to_string_pretty(json).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("JSON serialize error: {}", e),
        )
    })?;
    tmp.write_all(text.as_bytes())?;
    tmp.flush()?;
    tmp.persist(dest_path).map_err(|e| e.error);
    Ok(())
}

fn print_usage_and_exit(program: &str) -> ! {
    eprintln!(
        "Usage: {} <json-file> <dotted-path> <new-value-as-string>\n\
         Example: {} config.json a.b.c 42\n\
         Example bool: {} config.json enabled true\n\
         For arrays/objects, new-value should be valid JSON, e.g. '[1,2]' or '{{\"k\":1}}'",
        program, program, program
    );
    std::process::exit(2);
}

/*
/// 按点分割路径设置 Value 中的键，例如 "a.b.c"
/// 返回是否成功设置（即路径存在并被替换，或最后一级创建并赋值）
/// 说明：如果你希望仅在路径已存在时才修改，可以修改逻辑检测中间节点是否存在且为对象。
fn set_json_value(root: &mut Value, dotted_path: &str, new_val: Value) -> bool {
    let parts: Vec<&str> = dotted_path.split('.').filter(|s| !s.is_empty()).collect();
    if parts.is_empty() {
        // 空路径：不能设置整个文档（你可以选择允许）
        return false;
    }

    let mut cur = root;
    for (i, key) in parts.iter().enumerate() {
        let is_last = i + 1 == parts.len();
        match cur {
            Value::Object(map) => {
                if is_last {
                    // 设置最后一级为 new_val
                    map.insert(key.to_string(), new_val);
                    return true;
                } else {
                    // 下降到下一层；如果不存在，就创建空对象以便继续
                    if !map.contains_key(*key) {
                        map.insert(key.to_string(), Value::Object(serde_json::Map::new()));
                    }
                    cur = map.get_mut(*key).unwrap();
                }
            }
            // 如果当前不是对象但不是最后一级，无法继续
            _ => return false,
        }
    }
    false
}

/// 原子写回 JSON：先写入临时文件（同一目录），然后重命名覆盖原文件
/// 这样可以避免写入中断导致文件损坏
fn write_json_atomic<P: AsRef<Path>>(dest: P, json: &Value) -> io::Result<()> {
    let dest_path = dest.as_ref();
    let parent = dest_path.parent().unwrap_or_else(|| Path::new(".")); // 临时文件放在同目录保证同一文件系统以便重命名

    let mut tmp = NamedTempFile::new_in(parent)?;
    // 这里使用 pretty formatter，若你想保持原文件的压缩格式可使用 serde_json::to_string(json)
    let text = serde_json::to_string_pretty(json).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("JSON serialize error: {}", e),
        )
    })?;
    tmp.write_all(text.as_bytes())?;
    tmp.flush()?;
    // persist 替换目标文件（移动/重命名）
    tmp.persist(dest_path).map_err(|e| e.error).unwrap();
    Ok(())
}
*/
