#include "tree_sitter/parser.h"

#define LANGUAGE_VERSION 14
#define STATE_COUNT 2
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 3
#define ALIAS_COUNT 0
#define TOKEN_COUNT 1
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 1
#define MAX_RESERVED_WORD_SET_SIZE 0
#define PRODUCTION_ID_COUNT 0

enum {
  sym_text = 1,
  sym_source_file = 2,
};

static const char *ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_text] = "text",
  [sym_source_file] = "source_file",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_text] = sym_text,
  [sym_source_file] = sym_source_file,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym_text] = {
    .visible = true,
    .named = true,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  
  if (lexer->eof(lexer)) {
    return false;
  }
  
  // Consume all characters as text
  lexer->result_symbol = sym_text;
  
  // Read until EOF or EOL
  while (!lexer->eof(lexer)) {
    lexer->advance(lexer, false);
  }
  
  return true;
}

TS_PUBLIC const TSLanguage *tree_sitter_mage(void) {
  static const TSLanguage language = {
    .abi_version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .symbol_metadata = ts_symbol_metadata,
    .lex_fn = ts_lex,
    .symbol_names = ts_symbol_names,
    .public_symbol_map = ts_symbol_map,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .name = "mage",
  };
  return &language;
} 