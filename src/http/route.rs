use crate::types::{HttpRequest, ParamType, Route};

impl Route {
    // Check if the route matches the request based on the request parts and the route parts e.g /example/123 should match /example/:id
    pub fn path_matches(&self, request: &HttpRequest) -> bool {
        let route_parts: Vec<&str> = self.path.split('/').collect();
        let request_parts: Vec<&str> = request.request_lines.path.split('/').collect();

        if route_parts.len() != request_parts.len() {
            return false;
        }

        for (route_part, request_part) in route_parts.iter().zip(request_parts.iter()) {
            if let Some((_, param_type)) = parse_parameter(route_part) {
                match param_type {
                    ParamType::String => {}

                    ParamType::U16 => {
                        if request_part.parse::<u16>().is_err() {
                            return false;
                        }
                    }
                }

                continue;
            }

            if route_part != request_part {
                return false;
            }
        }

        true
    }
}

fn parse_parameter(route_part: &str) -> Option<(&str, ParamType)> {
    if !route_part.starts_with(':') {
        return None;
    }

    let parameter = &route_part[1..];

    if let Some((name, type_part)) = parameter.split_once('<') {
        let type_part = type_part.strip_suffix('>')?;

        let param_type = match type_part {
            "u16" => ParamType::U16,
            _ => return None,
        };

        if name.is_empty() {
            return None;
        }

        Some((name, param_type))
    } else {
        if parameter.is_empty() {
            return None;
        }

        Some((parameter, ParamType::String))
    }
}
