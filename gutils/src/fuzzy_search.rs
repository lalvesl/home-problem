use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{
    DocAddress, Document, Index, IndexWriter, ReloadPolicy, Score, TantivyError,
};

pub struct TextIndex {
    index: Index,
    schema: Schema,
    index_writer: IndexWriter,
}

impl TextIndex {
    pub fn new() -> tantivy::Result<Self> {
        // Create schema
        let mut schema_builder = Schema::builder();
        let text_field = schema_builder.add_text_field("text", TEXT | STORED);
        let schema = schema_builder.build();

        // Create index in memory
        let index = Index::create_in_ram(schema.clone());
        let index_writer = index.writer(50_000_000)?; // 50MB

        Ok(TextIndex {
            index,
            schema,
            index_writer,
        })
    }

    pub fn add_text(&mut self, text: &str) -> tantivy::Result<()> {
        let text_field = self.schema.get_field("text").unwrap();
        let mut doc = TantivyDocument::default();
        doc.add_text(text_field, text);
        self.index_writer.add_document(doc);
        Ok(())
    }

    pub fn commit(&mut self) -> tantivy::Result<()> {
        self.index_writer.commit()?;
        Ok(())
    }

    pub fn search(
        &self,
        query_str: &str,
        limit: usize,
    ) -> tantivy::Result<Vec<(Score, String)>> {
        let reader = self
            .index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommit)
            .try_into()?;
        let searcher = reader.searcher();

        let text_field = self.schema.get_field("text").unwrap();
        let query_parser =
            QueryParser::for_index(&self.index, vec![text_field]);

        let query = query_parser.parse_query(query_str)?;
        let top_docs = searcher.search(&query, &TopDocs::with_limit(limit))?;

        let mut results = Vec::new();

        for (score, doc_address) in top_docs {
            let retrieved_doc = searcher.doc(doc_address)?;
            let text_values: Vec<&str> = retrieved_doc
                .get_all(text_field)
                .map(|v| v.text().unwrap_or(""))
                .collect();
            for text in text_values {
                results.push((score, text.to_string()));
            }
        }

        Ok(results)
    }
}
