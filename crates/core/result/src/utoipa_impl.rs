use utoipa::{Modify, openapi::{Content, OpenApi, Ref, RefOr, ResponseBuilder}};

pub struct ErrorAddon;

impl Modify for ErrorAddon {
    fn modify(&self, utoipa: &mut OpenApi) {
        let response = ResponseBuilder::new()
            .description("An error occurred")
            .content(
                "application/json",
                Content::new(Some(RefOr::Ref(Ref::from_schema_name("Error")))),
            )
            .build();

        for path in utoipa.paths.paths.values_mut() {
            if let Some(route) = path.get.as_mut() {
                route
                    .responses
                    .responses
                    .insert("default".to_string(), RefOr::T(response.clone()));
            };

            if let Some(route) = path.delete.as_mut() {
                route
                    .responses
                    .responses
                    .insert("default".to_string(), RefOr::T(response.clone()));
            };

            if let Some(route) = path.patch.as_mut() {
                route
                    .responses
                    .responses
                    .insert("default".to_string(), RefOr::T(response.clone()));
            };

            if let Some(route) = path.post.as_mut() {
                route
                    .responses
                    .responses
                    .insert("default".to_string(), RefOr::T(response.clone()));
            };

            if let Some(route) = path.put.as_mut() {
                route
                    .responses
                    .responses
                    .insert("default".to_string(), RefOr::T(response.clone()));
            };
        }
    }
}
