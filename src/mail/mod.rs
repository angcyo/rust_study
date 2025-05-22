use mail_send::mail_builder::MessageBuilder;
use mail_send::SmtpClientBuilder;

///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2025/05/22
///
/// 发送邮件
/// [from] 发送者 ("angcyo", "angcyo@126.com")
/// [to] 接收者 ("angcyo", "angcyo@126.com")
/// [title] 标题
/// [html_body] html 内容
/// [text_body] text 内容
///
/// [host] smtp 服务器 和 端口
/// [credentials] 用户名 和 密码
///
/// https://crates.io/crates/mail-send
pub async fn send_mail(
    from: (String, String),
    to: (String, String),
    title: &str,
    html_body: &str,
    text_body: &str,
    //--
    host: (&str, u16),
    credentials: (&str, &str),
) {
    // Build a simple multipart message
    let message = MessageBuilder::new()
        .from(from)
        .to(to)
        .subject(title)
        .html_body(html_body)
        .text_body(text_body);

    // Connect to the SMTP submissions port, upgrade to TLS and
    // authenticate using the provided credentials.
    SmtpClientBuilder::new(host.0, host.1)
        .implicit_tls(false) //starttls or ssl
        .credentials(credentials)
        .timeout(std::time::Duration::from_secs(5))
        .connect()
        .await
        .unwrap()
        .send(message)
        .await
        .unwrap();
}
