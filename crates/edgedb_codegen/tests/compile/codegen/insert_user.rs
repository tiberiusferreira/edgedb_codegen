fn main() {
	pub mod example {
		use ::edgedb_codegen::exports as e;
		#[doc = r" Execute the desired query."]
		#[cfg(feature = "query")]
		pub async fn query(
			client: &e::edgedb_tokio::Client,
			props: &Input,
		) -> core::result::Result<Output, e::edgedb_errors::Error> {
			client.query_required_single(QUERY, props).await
		}
		#[doc = r" Compose the query as part of a larger transaction."]
		#[cfg(feature = "query")]
		pub async fn transaction(
			conn: &mut e::edgedb_tokio::Transaction,
			props: &Input,
		) -> core::result::Result<Output, e::edgedb_errors::Error> {
			conn.query_required_single(QUERY, props).await
		}
		#[derive(Clone, Debug, e :: typed_builder :: TypedBuilder)]
		#[cfg_attr(feature = "serde", derive(e::serde::Serialize, e::serde::Deserialize))]
		#[cfg_attr(feature = "query", derive(e::edgedb_derive::Queryable))]
		pub struct Input {
			#[builder(setter(into))]
			pub name: String,
			#[builder(setter(into))]
			pub bio: String,
			#[builder(setter(into))]
			pub slug: String,
		}
		impl e::edgedb_protocol::query_arg::QueryArgs for Input {
			fn encode(
				&self,
				encoder: &mut e::edgedb_protocol::query_arg::Encoder,
			) -> core::result::Result<(), e::edgedb_errors::Error> {
				let map = e::edgedb_protocol::named_args! { "name" => self . name . clone () , "bio" => self . bio . clone () , "slug" => self . slug . clone () , };
				map.encode(encoder)
			}
		}
		#[derive(Clone, Debug, e :: typed_builder :: TypedBuilder)]
		#[cfg_attr(feature = "serde", derive(e::serde::Serialize, e::serde::Deserialize))]
		#[cfg_attr(feature = "query", derive(e::edgedb_derive::Queryable))]
		pub struct Output {
			#[builder(setter(into))]
			pub id: e::uuid::Uuid,
			# [builder (default , setter (into , strip_option (fallback = name_opt)))]
			pub name: Option<String>,
			# [builder (default , setter (into , strip_option (fallback = bio_opt)))]
			pub bio: Option<String>,
			#[builder(setter(into))]
			pub slug: String,
		}
		#[doc = r" The original query string provided to the macro. Can be reused in your codebase."]
		pub const QUERY: &str = "with NewUser := (insert User {\n  name := <str>$name,\n  bio := \
		                         <str>$bio,\n  slug := <str>$slug,\n})\nselect NewUser {\n  id,\n  \
		                         name,\n  bio,\n  slug,\n};\n";
	}
}
