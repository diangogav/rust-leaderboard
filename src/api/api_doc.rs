use rocket::fairing::AdHoc;
use rocket_okapi::rapidoc::make_rapidoc;
use rocket_okapi::rapidoc::GeneralConfig;
use rocket_okapi::rapidoc::HideShowConfig;
use rocket_okapi::rapidoc::RapiDocConfig;
use rocket_okapi::settings::UrlObject;
use rocket_okapi::swagger_ui::make_swagger_ui;
use rocket_okapi::swagger_ui::SwaggerUIConfig;

pub fn run() -> AdHoc {
    AdHoc::on_ignite("Documentation created!", |rocket| async {
        rocket
            .mount(
                "/swagger/",
                make_swagger_ui(&SwaggerUIConfig {
                    url: "../openapi.json".to_owned(),
                    ..Default::default()
                }),
            )
            .mount(
                "/rapidoc/",
                make_rapidoc(&RapiDocConfig {
                    title: Some("RapiDoc".to_owned()),
                    general: GeneralConfig {
                        spec_urls: vec![UrlObject::new("General", "../openapi.json")],
                        ..Default::default()
                    },
                    hide_show: HideShowConfig {
                        allow_spec_url_load: false,
                        allow_spec_file_load: false,
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            )
    })
}
