#include <stdint.h>

typedef struct ext_handler_registration {
  void* _self;
  void (*deregister)(void*);
} ext_handler_registration;

typedef struct _ext_click_handler {
  void* _self;
  void (*on_click)(void*);
} ext_click_handler;

typedef struct ext_has_click_handlers {
  void* _self; 
  ext_handler_registration (*add_handler)(void*, ext_click_handler);
} ext_has_click_handlers;

typedef struct ext_text {
  int32_t length;
  uint8_t const* content;
} ext_text;

typedef struct ext_has_text {
  void* _self;
  ext_text (*get_text)(void*);
  void (*set_text)(void*, ext_text);
} ext_has_text;

typedef struct ext_button {
  ext_has_click_handlers click_handlers;
  ext_has_text text;
} ext_button;

typedef struct ext_main_menu_view {
  void* _self;
  ext_button start_game_button;
} ext_main_menu_view;

typedef struct ext_main_menu_presenter {
  void* _self;
} ext_main_menu_presenter;

ext_main_menu_presenter bind_main_menu_view(ext_main_menu_view);
uint8_t* allocate_string(int32_t);
