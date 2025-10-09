### Aim

The aim of this project is build an AI Songwriter, with humans using their taste in music to customize generation and select possible options. There are multiple elements involved in creating a song which resonates with audiences - lyrics, beats, artists, theme of song etc.

This project is a fun project which for now, focuses on the lyrics generation part, the aim is that it is very tailored. Tailored by genre, tailored by artist style, tailored by message of the music. An interesting extension will be the integration of beats, sampling of beats and analysis.

Proposed Features (Ranked by order of implementation):

    - Enable Keyword Search and Semantic Search of Lyrics (filtering by select artists, or all the database ) 
    - Generating lyrics from narrative/descriptive experiences, journal entries. Ability to shift emphasis across topics in the experience. One-person lyrics, Two-person conversational lyrics.
    - Style Transfer across select artists, select songs, select playlists ( Integration with Spotify to access created playlists by a user ).
    - Suggesting song titles from generated lyrics (varying styles - inclusivity).
    - Generate variants from a song i.e covers but maintain flow/rhythm(depends on beat integration)

    - Adapt to multilingual ( French, Yoruba ).


### Technical Complexity 

- Obtaining Text Lyrics: 

    - Clean curated lyrics from sources like musixmatch, Genius APIs.
    - Whisper Text to Speech Models ( Native Models for multilingual transcription ) - 
        how to evaluate performance? - compare speech-to-text model transcription with clean curated lyrics.
        finetune speech-to-text model with a sample, and deploy for each categorization(usually for new/unindexed lyric, enable user to annotate and correct?).
    - Identifying breaks between lines for lyrics.

- Obtaining Additional Metadata

    - Genres, etc can be obtained from trusted sources
    - Otherwise classification models to predict genres can get complex really fast
    
    Gold standard annotated genres is the way to go.

- Display(Frontend)
    - The option of using WASM means heavy css customization for a music themed website. This may mean more time is spent building components rather than leverage existing ones. Trade off is the reduced computational overhead.

    - React/Typescript however can be useful for heavy consumer facing websites. Managing State is convenient, with standard approaches

- Storing Text Lyrics

    - Sqlite/Postgres enable filtering/categorization by information available in columns
    - ElasticSearch for integrated semantic search 

    hmmm?

- Keyword Search

    Databases these days have implemented full-text search in the capabilities of databases, while Elasticsearch have implemented
    semantic search into the capabilities of databases

- Preparing the Dataset

    - Python transformation script where cleanly curated lyrics is the target,
    
    how do we obtain the source?
    hmmm?

- Experimentation Interface

    - Weights and Biases/ML flow to track eval metrics for finetuning/training workflow

- Generating Lyrics

    - Finetuning a Seq2Seq Model, 
    - Finetuning an instruct large language model using (human - assistant pair).
    - Training a model from scratch using masked language modelling, causal language modelling 
