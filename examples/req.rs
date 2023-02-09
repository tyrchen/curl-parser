use anyhow::Result;

fn main() -> Result<()> {
    let input = r#"curl \
    -X PATCH \
    -d '{"visibility":"private"}' \
    -H "Accept: application/vnd.github+json" \
    -H "Authorization: Bearer <YOUR-TOKEN>"\
    -H "X-GitHub-Api-Version: 2022-11-28" \
    https://api.github.com/user/email/visibility "#;
    let parsed = curl_parser::ParsedRequest::try_from(input)?;
    println!("{parsed:#?}");
    Ok(())
}
