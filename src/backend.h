#include <stdbool.h>

enum Direction { UP, DOWN, LEFT, RIGHT };
enum Status { SUCCESS, FAILURE };
struct PointList {
  int x;
  int y;
  struct PointList* next;
};

typedef struct PointList PointList;

typedef struct {
  PointList* snake;
  PointList* foods;
  int xmax;
  int ymax;
} Board;


extern bool is_same_place(PointList* cell1, PointList* cell2);
enum Status move_snake(Board* board, enum Direction dir);
PointList* next_move(Board* board, enum Direction dir);
extern PointList* create_cell(int x, int y);
extern PointList* create_random_cell(int xmax, int ymax);
extern PointList* create_snake();
extern Board* create_board(PointList* foods, PointList* snake, int xmax, int ymax);
extern bool list_contains(PointList* cell, PointList* list);
bool remove_from_list(PointList* elt, PointList** list);
extern void add_new_food(Board* board);
