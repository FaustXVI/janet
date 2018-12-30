# Define CMake's minimum version (must-do) and the project's name and supported languages
#set(CMAKE_ASM_COMPILER avrdude)
cmake_minimum_required(VERSION 3.8)
project(Blink LANGUAGES C CXX ASM)

# Call a framework utility function, passing it information about the hardware board that will
# be used - This function returns a structure known only to the framework
get_board_id(board_id uno)

# Create an executable suitable for the Arduino firmware using CMake-style target-creation
add_arduino_executable(Blink ${board_id} src/Blink.cpp)

#add_arduino_example(my_example_target_name ${board_id} Blink)

# Upload the created target through a connected Serial Port (Where your board is connected to)
upload_arduino_target(Blink "${board_id}" /dev/ttyACM0)
