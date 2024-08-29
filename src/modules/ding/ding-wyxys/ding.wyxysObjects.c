#include <stdio.h>
#include <string.h>

// Define a structure for an object
typedef struct {
    int intValue;
    float floatValue;
    double doubleValue;
    char charValue;
    char stringValue[100];
} Object;

// Function to initialize the object
void initializeObject(Object* obj, int intVal, float floatVal, double doubleVal, char charVal, const char* strVal) {
    obj->intValue = intVal;
    obj->floatValue = floatVal;
    obj->doubleValue = doubleVal;
    obj->charValue = charVal;
    strcpy(obj->stringValue, strVal);
}

// Function to print the object properties
void printObject(const Object* obj) {
    printf("Integer Value: %d\n", obj->intValue);
    printf("Float Value: %.2f\n", obj->floatValue);
    printf("Double Value: %.5f\n", obj->doubleValue);
    printf("Character Value: %c\n", obj->charValue);
    printf("String Value: %s\n", obj->stringValue);
}

int main() {
    // Create an object
    Object gSys34;

    // Initialize the object
    initializeObject(&gSys34, 10, 3.14f, 2.71828, 'X', "%c@@v");

    // Print the object properties
    printObject(&gSys34);

    return 0;
}
