# Fine-Tuning for Scientific Paper Summarizing
This project fine-tunes the Google Pegasus model on the SciTLDR dataset to generate summaries of scientific papers. Data preprocessing is implemented in Rust (`preprocessing/`), while fine-tuning and evaluation live in a Python notebook (`finetune/main.ipynb`).

**Repository layout**
- `preprocessing/`: Rust pipeline for converting SciTLDR JSONL into chunked inputs.
- `finetune/main.ipynb`: End-to-end fine-tuning, evaluation, and inference.
- `finetune/pegasus-FullText/`: Saved checkpoints and trainer logs.

## Process summary
1. **Preprocess (Rust)**: Combine sentence lists into full text, tokenize with Pegasus SentencePiece, then chunk into 1024-token windows with 512-token overlap. Output JSONL stores `source` as a list of chunks and `target` as a single summary string.
2. **Flatten (Notebook)**: Convert each document into multiple training examples (one per chunk), while keeping `doc_id` and `chunk_id`.
3. **Fine-tune (Notebook)**: Train `google/pegasus-large` for 3 epochs with 1024-token inputs and 128-token summaries.
4. **Evaluate (Notebook)**:
   - **Chunk-level eval during training** (flattened test set) at step 4545: ROUGE-1 24.86, ROUGE-2 6.43, ROUGE-L 17.55, ROUGE-Lsum 17.56.
   - **Document-level eval** (summarize chunks, then summarize the concatenated chunk summaries) on 618 test docs: ROUGE-1 37.85, ROUGE-2 12.36, ROUGE-L 22.92, ROUGE-Lsum 23.00.

## Data expectations
The pipeline assumes the SciTLDR FullText split in JSONL format:
```
data/
  raw/
    SciTLDR-FullText/
      train.jsonl
      test.jsonl
  processed/
    SciTLDR-FullText/
      train.jsonl
      test.jsonl
```
Each raw line is expected to match the schema used in `preprocessing/src/main.rs`:
```
{"source": ["sentence1", "sentence2", ...], "target": ["summary sentence1", ...]}
```

## Usage

### 1) Preprocess (Rust)
The Rust binary reads the raw JSONL, tokenizes with Pegasus SentencePiece, and writes the chunked JSONL output.

```
cd preprocessing
cargo run --release
```

Notes:
- Input/output paths are currently hardcoded in `preprocessing/src/main.rs` (`READ_PATH` and `WRITE_PATH`).
- The script processes one split at a time (currently set to `train.jsonl`). Update the paths to also process `test.jsonl`.
- The SentencePiece model is loaded from `preprocessing/models/pegasus/spiece.model`.

### 2) Fine-tune + evaluate (Python)
Open and run the notebook:
```
finetune/main.ipynb
```

Key details from the notebook:
- **Model**: `google/pegasus-large`
- **Tokenization**: `max_input_len=1024`, `max_target_len=128`
- **Training**: batch size 2, gradient accumulation 8, learning rate 5e-5, 3 epochs
- **Outputs**: checkpoints written to `finetune/pegasus-FullText/`

The notebook also includes an inference helper (`generate_summary`) that summarizes each chunk and then summarizes the concatenated chunk summaries into a final document summary.

## Results
The latest recorded metrics from the notebook are:
- **Training-time chunk-level eval** (flattened test set, step 4545): ROUGE-1 24.86, ROUGE-2 6.43, ROUGE-L 17.55, ROUGE-Lsum 17.56.
- **Document-level eval** (hierarchical chunk summarization on 618 test docs): ROUGE-1 37.85, ROUGE-2 12.36, ROUGE-L 22.92, ROUGE-Lsum 23.00.

Checkpoint artifacts and logs are available under `finetune/pegasus-FullText/`.
