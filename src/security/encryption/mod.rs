use config::types::security_types::ApplicationHasher;
use config::types::security_types::HashResult;
use config::ResourceId;

pub mod hash {
	pub struct hash_wrapper {
		hasher: ApplicationHasher,
	}

	pub fn create_new_hasher() -> ApplicationHasher {
		return <ApplicationHasher as Digest>::new()
	}

	impl hash_wrapper {
		pub fn new() -> hash_wrapper {
			return hash_wrapper {
				hasher: create_new_hasher()
			}
		}

		pub fn hash_data(&mut self, data: impl AsRef<[u8]>) -> HashResult {
			self.hasher.reset();
			self.hasher.update(data);
			return self.hasher.finalize();
		}
	}
}