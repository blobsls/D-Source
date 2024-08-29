#include <stdio.h>
#include <ctype.h>
#include <string.h>

// Define token types
typedef enum {
    TOKEN_IDENTIFIER,
    TOKEN_NUMBER,
    TOKEN_OPERATOR,
    TOKEN_KEYWORD,
    TOKEN_EOF,
    // Add more token types as needed
} TokenType;

// Define a token structure
typedef struct {
    TokenType type;
    char value[256];
} Token;

// Define a lexer structure
typedef struct {
    const char *input;
    size_t pos;
} Lexer;

// Function to initialize the lexer
void lexer_init(Lexer *lexer, const char *input) {
    lexer->input = input;
    lexer->pos = 0;
}

// Function to get the next character
char lexer_next_char(Lexer *lexer) {
    return lexer->input[lexer->pos++];
}

// Function to peek at the next character
char lexer_peek_char(Lexer *lexer) {
    return lexer->input[lexer->pos];
}

// Function to recognize identifiers and keywords
Token lexer_next_token(Lexer *lexer) {
    Token token;
    char c = lexer_next_char(lexer);

    // Skip whitespace
    while (isspace(c)) {
        c = lexer_next_char(lexer);
    }

    // Recognize identifiers and keywords
    if (isalpha(c) || c == '_') {
        size_t start = lexer->pos - 1;
        while (isalnum(lexer_peek_char(lexer)) || lexer_peek_char(lexer) == '_') {
            lexer_next_char(lexer);
        }
        size_t length = lexer->pos - start;
        strncpy(token.value, lexer->input + start, length);
        token.value[length] = '\0';
        token.type = TOKEN_IDENTIFIER;
        // Check if the identifier is a keyword (e.g., "function", "var")
        if (strcmp(token.value, "function") == 0 || strcmp(token.value, "var") == 0) {
            token.type = TOKEN_KEYWORD;
        }
        return token;
    }

    // Recognize numbers
    if (isdigit(c)) {
        size_t start = lexer->pos - 1;
        while (isdigit(lexer_peek_char(lexer))) {
            lexer_next_char(lexer);
        }
        size_t length = lexer->pos - start;
        strncpy(token.value, lexer->input + start, length);
        token.value[length] = '\0';
        token.type = TOKEN_NUMBER;
        return token;
    }

    // Recognize operators
    if (strchr("+-*/=", c)) {
        token.value[0] = c;
        token.value[1] = '\0';
        token.type = TOKEN_OPERATOR;
        return token;
    }

    // End of input
    token.type = TOKEN_EOF;
    token.value[0] = '\0';
    return token;
}

// Main function for testing
int main() {
    const char *code = "function test(var x) { return x + 1; }";
    Lexer lexer;
    lexer_init(&lexer, code);

    Token token;
    do {
        token = lexer_next_token(&lexer);
        printf("Token: %s (Type: %d)\n", token.value, token.type);
    } while (token.type != TOKEN_EOF);

    return 0;
}
