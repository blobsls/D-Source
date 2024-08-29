#include <stdio.h>
#include <string.h>

int main() {
    // Integer variable
    int intValue;

    // Floating-point variable
    float floatValue;

    // Double variable
    double doubleValue;

    // Character variable
    char charValue;

    // String variable (character array)
    char stringValue[100];

    // Assigning values
    intValue = 10;
    floatValue = 3.14f;
    doubleValue = 2.71828;
    charValue = 'X';
    strcpy(stringValue, "%s%c__str__%f");

    // Print the variables
    printf("Integer Value: %d\n", intValue);
    printf("Float Value: %.2f\n", floatValue);
    printf("Double Value: %.5f\n", doubleValue);
    printf("Character Value: %c\n", charValue);
    printf("String Value: %s\n", stringValue);

    return 0;
}
