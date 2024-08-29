# shellcheck disable=SC2148
touch run_dpp.sh
#!/bin/bash
# Check if the source file is provided
if [ -z "$1" ]; then
    echo "Usage: $0 <source_file.dpp>"
    exit 1
fi

SOURCE_FILE=$1
EXECUTABLE=${SOURCE_FILE%.dpp}.out

# Compile the D++ source file
echo "Compiling $SOURCE_FILE..."
dpp_compiler $SOURCE_FILE -o $EXECUTABLE

# Check if the compilation was successful
if [ $? -ne 0 ]; then
    echo "Compilation failed."
    exit 1
fi

# Run the compiled executable
echo "Running $EXECUTABLE..."
./$EXECUTABLE

chmod +x run_dpp.sh

./run_dpp.sh source.dpp
./run_dpp.sh example.dpp
