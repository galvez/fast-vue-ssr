
pub struct RendererRequest {
    pub url: String,
    pub method: String,
    pub headers: String,
    pub async_data: String,
}

impl RendererRequest {
	pub fn new(url: String, method: String, headers: String, async_data: String) -> RendererRequest {
        RendererRequest {
        	url,
        	method,
        	headers,
        	async_data,
        }
	}
}
