use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "pages/home-page.html")]
pub struct HomePage<'a> {
  pub name: &'a str
}
