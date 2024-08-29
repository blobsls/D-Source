#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Function prototype declarations
void appendArrowSegment(char **dest, const char *segment, size_t *currentLength, size_t *maxLength);
void generateArrowString(char **result, size_t *length);

int main() {
    char *arrowString = NULL;
    size_t arrowLength = 0;

    // Generate the arrow string
    generateArrowString(&arrowString, &arrowLength);

    // Print the generated arrow string
    printf("Arrow String:\n%s\n", arrowString);

    // Clean up memory
    if (arrowString != NULL) {
        free(arrowString);
    }

    return 0;
}

void appendArrowSegment(char **dest, const char *segment, size_t *currentLength, size_t *maxLength) {
    size_t segmentLength = strlen(segment);

    if (*currentLength + segmentLength + 1 > *maxLength) {
        *maxLength = (*currentLength + segmentLength + 1) * 2;
        *dest = realloc(*dest, *maxLength);
        if (*dest == NULL) {
            fprintf(stderr, "Memory allocation error!\n");
            exit(EXIT_FAILURE);
        }
    }

    strcat(*dest, segment);
    *currentLength += segmentLength;
}

void generateArrowString(char **result, size_t *length) {
    const char *arrowComponents[] = {
        "<--", "-->", "<->", "<|>", "<<>>", "==>", "<==", "><", "<<-->>"
    };
    size_t numComponents = sizeof(arrowComponents) / sizeof(arrowComponents[0]);

    *length = 0;
    size_t maxLength = 1024; // Initial allocation size
    *result = malloc(maxLength);

    if (*result == NULL) {
        fprintf(stderr, "Memory allocation error!\n");
        exit(EXIT_FAILURE);
    }

    (*result)[0] = '\0'; // Initialize empty string

    int i, j;
    for (i = 0; i < 100; i++) { // Repeat to make it super long
        for (j = 0; j < numComponents; j++) {
            appendArrowSegment(result, arrowComponents[j], length, &maxLength);
            appendArrowSegment(result, "---", length, &maxLength); // separator
        }
    }
}
