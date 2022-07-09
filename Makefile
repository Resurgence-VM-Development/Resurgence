CXX         := g++

WARNINGS 	:= 	-Werror						\
				-Wfatal-errors				\
				-Wall 						\
				-Wextra 					\
				-pedantic 					\
				-Wshadow					\
				-Wnon-virtual-dtor			\
				-Wold-style-cast			\
				-Wcast-align				\
				-Wunused					\
				-Woverloaded-virtual		\
				-Wpedantic					\
				-Wconversion				\
				-Wsign-conversion			\
				-Wmisleading-indentation	\
				-Wduplicated-cond			\
				-Wduplicated-branches		\
				-Wlogical-op				\
				-Wnull-dereference			\
				-Wuseless-cast				\
				-Wdouble-promotion			\
				-Wformat=2

CXX_FLAGS   := -std=c++20 -static -static-libgcc -static-libstdc++ -DFMT_HEADER_ONLY

BIN         := bin
INCLUDE     := -Iinclude

SRC         := src/RendorInterpreter

OBJDIR      := bin/object_files
EXECUTABLE  := surge

SOURCES     := $(wildcard $(SRC)/*.cpp) $(wildcard $(SRC)/**/*.cpp)
OBJECTS     := $(patsubst $(SRC)/%.cpp,$(OBJDIR)/%.o,$(SOURCES))

all: $(BIN)/$(EXECUTABLE)

run: clean all
	clear
	./$(BIN)/$(EXECUTABLE)

$(BIN)/$(EXECUTABLE): $(OBJECTS)
	@echo 
	$(CXX) $(CXX_FLAGS) $(INCLUDE) $^ -o $@ -Llib -lbinary_io
	@echo 

$(OBJDIR)/%.o: $(SRC)/%.cpp 
	$(CXX) $(CXX_FLAGS) $(WARNINGS) $(INCLUDE) $^ -c -o $@ 

$(OBJDIR)/**/%.o: $(SRC)/**/%.cpp 
	$(CXX) $(CXX_FLAGS) $(WARNINGS) $(INCLUDE) $^ -c -o $@ 

clean:
	-rm $(OBJDIR)/*.o
	-rm $(OBJDIR)/**/*.o