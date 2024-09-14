use anyhow::{anyhow, Result};
use clap::Parser;
use colored::*;
use mime::Mime;
use reqwest::{header, Client, Response, Url};
use std::{collections::HashMap, str::FromStr};

use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

// 定义 HTTPie 的 CLI 的子命令 目前只支持 `get` 和 `post`
#[derive(Parser, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

// 定义 HTTPie 的 CLI 的主入口，它包含若干个子命令
// 下面 /// 的注释是文档，clap 会将其作为 CLI 的帮助

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "qgy")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

// get 子命令

/// feed get with an url and we will retrieve the response for you

#[derive(Parser, Debug)]
struct Get {
    /// HTTP 请求的URL
    #[clap(parse(try_from_str=parse_url))]
    url: String,
}

// post 子命令。需要输入一个 URL，和若干个可选的 key=value，用于提供 json body
/// feed post with an url and optional key=value pairs. We will post the data
/// as JSON, and retrieve the response for you

#[derive(Parser, Debug)]
struct Post {
    /// HTTP 请求的 URL
    #[clap(parse(try_from_str=parse_url))]
    url: String,

    /// key=value pairs 组成的 Vec
    #[clap(parse(try_from_str=parse_kv_pair))]
    body: Vec<KvPair>,
}

#[derive(Debug, PartialEq)]
struct KvPair {
    k: String,
    v: String,
}

/// 当我们实现 FromStr trait 后，可以用 str.parse() 方法将字符串解析成 KvPair
impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            k: (split.next().ok_or_else(err)?).to_string(),
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    Ok(print_resp(resp).await?)
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for kv in args.body.iter() {
        body.insert(&kv.k, &kv.v);
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    Ok(print_resp(resp).await?)
}
// 打印状态码
fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n", status);
}

// 打印 header
fn print_headers(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}: {}", name.to_string().green(), value.to_str().unwrap());
    }
    print!("\n");
}

// 打印 body
fn print_body(m: Option<Mime>, body: &String) {
    match m {
        Some(v) if v == mime::APPLICATION_JSON => {
            // &jsonxf::pretty_print(body).unwrap().cyan().to_string()
            highlight_code(body, "json");
        }
        _ => highlight_code(body, "html"),
    }
}

fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

fn highlight_code(code: &str, ext: &str) {
    // 加载主题和语法
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    // 获取 Rust 语法和主题
    let syntax = ps.find_syntax_by_extension(ext).unwrap();
    let theme = &ts.themes["base16-ocean.dark"];

    // 创建实例
    let mut h = HighlightLines::new(syntax, theme);

    for line in LinesWithEndings::from(code) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        print!("{}", escaped);
    }
}

async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    print_headers(&resp);
    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(mime, &body);
    Ok(())
}

// 标记异步
#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let mut headers = header::HeaderMap::new();
    // 为 http 加一些 http 头
    headers.insert("X-POWERED-BY", "Rust".parse()?);
    headers.insert(header::USER_AGENT, "Rust Httpie".parse()?);

    let client = Client::builder().default_headers(headers).build()?;
    let result = match opts.subcmd {
        SubCommand::Get(ref args) => get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_works() {
        assert!(parse_url("abc").is_err());
        assert!(parse_url("http://abc").is_ok());
        assert!(parse_url("https://httpbin.org/post").is_ok());
    }

    #[test]
    fn parse_kv_pair_works() {
        assert!(parse_kv_pair("a").is_err());
        assert_eq!(
            parse_kv_pair("a=1").unwrap(),
            KvPair {
                k: "a".to_string(),
                v: "1".to_string(),
            }
        );
        assert_eq!(
            parse_kv_pair("b=").unwrap(),
            KvPair {
                k: "b".into(),
                v: "".into(),
            }
        );
    }
}
