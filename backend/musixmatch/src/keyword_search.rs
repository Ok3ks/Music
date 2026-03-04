use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use seekstorm::index::{Index, open_index};
use clap::{ Subcommand, Parser};

struct SearchFn<'a> {
    path_to_index: &'a str,
}

impl SearchFn<'_> {

    pub async fn new(&self) -> Result<(), Box<dyn std::error::Error>> {
        use seekstorm::index::{
            AccessType, FrequentwordType, IndexMetaObject, NgramSet, SimilarityType, StemmerType,
            StopwordType, TokenizerType, create_index,
        };

        ///TODO
        let schema_json = r#"
        [{"field":"title","field_type":"Text","stored":false,"indexed":false},
        {"field":"body","field_type":"Text","stored":true,"indexed":true},
        {"field":"url","field_type":"Text","stored":false,"indexed":false}]"#;
        let schema = serde_json::from_str(schema_json).unwrap();
        let meta = IndexMetaObject {
            id: 0,
            name: "acho_index".into(),
            similarity: SimilarityType::Bm25f,
            tokenizer: TokenizerType::AsciiAlphabetic,
            stemmer: StemmerType::None,
            stop_words: StopwordType::None,
            frequent_words: FrequentwordType::English,
            ngram_indexing: NgramSet::NgramFF as u8,
            access_type: AccessType::Mmap,
            spelling_correction: None,
            query_completion: None,
        };
        let segment_number_bits1 = 11;
        let index_arc = create_index(
            Path::new(self.path_to_index),
            meta,
            &schema,
            &Vec::new(),
            11,
            false,
            None,
        )
        .await?;

        Ok(())

    }

    pub async fn index_documents(&self) -> Result<(),  Box<dyn std::error::Error>> {
        use seekstorm::index::open_index;
        use seekstorm::index::IndexDocuments;

        ///TODO
        let documents_json = r#"
        [{"title":"title1 test","body":"body1","url":"url1"},
        {"title":"title2","body":"body2 test","url":"url2"},
        {"title":"title3 test","body":"body3 test","url":"url3"}]"#;

        let documents_vec = serde_json::from_str(documents_json)?;
        let index_arc = open_index(Path::new(&self.path_to_index), false).await;
    
        match index_arc {
            Ok(index) => {
                index.index_documents(documents_vec).await;
                Ok(())
            }

            Err(error) => {
                Err(error.into())
            }
        }
        
        // Ok(())
    }

    pub async fn update_index(&self) -> Result<(), Box<dyn std::error::Error>> {
        use seekstorm::commit::Commit;
        use seekstorm::index::UpdateDocuments;

        ///TODO
        let id_document_vec_json = r#"
        [[1,{"title":"title1 test","body":"body1","url":"url1"}],
        [2,{"title":"title3 test","body":"body3 test","url":"url3"}]]"#;
        let id_document_vec = serde_json::from_str(id_document_vec_json).unwrap();
        let index_arc = open_index(Path::new(&self.path_to_index), false).await;
    
        match index_arc {
            Ok(index_arc) => {
                index_arc.update_documents(id_document_vec).await;
                index_arc.commit().await;
            }

            Err(error) => {
                // Err::<(error);
            }
        }

        Ok(())
    }

    pub async fn ingest_json(&self, json_file: &Path) -> Result<(), Box<dyn std::error::Error>> {
        use seekstorm::ingest::IngestJson;
        use std::path::Path;

        let index_arc = open_index(Path::new(self.path_to_index), false).await;


        match index_arc {
            Ok(mut index_arc) => {
                index_arc.ingest_json(json_file).await;
                println!("1111");
                println!("{}", self.path_to_index);
            }

            Err(error) => {
                // Err(error.into());
                println!("error: {}", error)
            }
        }

        Ok(())
    }

    ///For reset functionality
    pub async fn delete_index(&self) -> ()  {
        let index_arc = open_index(Path::new(self.path_to_index), false).await;
        
        match index_arc {
            Ok(index_arc) => {
                let _ = index_arc.write().await.delete_index();
            }

            Err(error) => {
                // Err(error.into());
            }
        }

        
    }

    pub async fn search_index(&self, query: String) {
        use seekstorm::highlighter::{Highlight, highlighter};
        use seekstorm::search::{
            QueryFacet, QueryRewriting, QueryType, ResultType, Search,
        };
        use std::collections::HashSet;

        let offset = 0;
        let length = 10;
        let query_type = QueryType::Intersection;
        let result_type = ResultType::TopkCount;
        let include_uncommitted = false;
        let field_filter = Vec::new();
        let query_facets = vec![QueryFacet::String16 {
            field: "town".to_string(),
            prefix: "".to_string(),
            length: u16::MAX,
        }];
        let facet_filter = Vec::new();

        ///TODO
        let result_sort = Vec::new();
        let index_arc = open_index(Path::new(self.path_to_index), false).await;
        match index_arc {
            Ok(index_arc) => {
            let result_object = index_arc
                .search(
                    query,
                    query_type,
                    offset,
                    length,
                    result_type,
                    include_uncommitted,
                    field_filter,
                    query_facets,
                    facet_filter,
                    result_sort,
                    QueryRewriting::SearchOnly,
                )
                .await;

            // ### display results

            let highlights: Vec<Highlight> = vec![Highlight {
                field: "body".to_owned(),
                name: String::new(),
                fragment_number: 2,
                fragment_size: 160,
                highlight_markup: true,
                ..Default::default()
            }];
            let highlighter =
                Some(highlighter(&index_arc, highlights, result_object.query_terms).await);
            let return_fields_filter = HashSet::new();
            let distance_fields = Vec::new();
            let index = index_arc.write().await;

        ///TODO: specify return type
            for result in result_object.results.iter() {
                let doc = index
                    .get_document(
                        result.doc_id,
                        false,
                        &highlighter,
                        &return_fields_filter,
                        &distance_fields,
                    )
                    .await
                    .unwrap();
                println!(
                    "result {} rank {} body field {:?}",
                    result.doc_id,
                    result.score,
                    doc.get("body")
                );
            }
            println!(
                "result counts {} {} {}",
                result_object.results.len(),
                result_object.result_count,
                result_object.result_count_total
            );

            println!(
                "{}",
                serde_json::to_string_pretty(&result_object.facets).unwrap()
            );
        }
        
            Err(error) => {
                // Err(error.into());
            }
        }

    }
}
#[derive(Parser, Debug)]
#[command(version, about = "Index documents")]
pub struct SearchCli {

    // #[arg(short, long, group = "input")]
    // pub path_to_json: Option<String>,

    #[arg(short, long, group = "input")]
    pub keyword_match: Option<String>,

    #[command(subcommand)]
    pub action: Action,
    
    }

#[derive(Subcommand, Debug)]
enum Action {
        Index { 
            path_to_file: String,
            },
        Search { keyword: String  }
    }

pub fn search() {

    let args = SearchCli::parse();

    match &args.action {
        Action::Index { path_to_file } => {
            let user_search =  SearchFn
                {   
                    path_to_index: "/Users/max/Code/Music/backend/musixmatch/",
                };
                let file_path = Path::new(&path_to_file);
                let _ = user_search.ingest_json(file_path);
        }

        Action::Search { keyword } => {
            let user_search =  SearchFn
                {   
                    path_to_index: "/Users/max/Code/Music/backend/musixmatch/",
                };
            user_search.search_index(keyword.to_string());
        }
}
}
