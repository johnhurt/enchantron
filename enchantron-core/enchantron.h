#ifndef enchantron_h
#define enchantron_h

#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct {
  void (*on_click)(void*);
  void (*drop)(void*);
} ext_click_handler;

typedef struct {
  void *(*new)(int64_t);
  int64_t (*get_length)(void*);
  uint8_t *(*get_content)(void*);
} ext_rust_string;

typedef struct {
  ext_click_handler click_handler;
  ext_rust_string rust_string;
} int_ui_binding;

typedef struct {
  void *event_bus;
  void *texture_loader;
  int_ui_binding internal_ui_binding;
} ext_application_context;

typedef struct {
  void *(*get_start_game_button)(void*);
  void (*transition_to_game_view)(void*);
  void (*destroy)(void*);
} ext_main_menu_view;

typedef struct {
  int64_t (*get_width)(void*);
  int64_t (*get_height)(void*);
  int64_t (*get_x)(void*);
  int64_t (*get_y)(void*);
  void (*destroy)(void*);
} ext_game_view;

typedef struct {
  void *(*add_click_handler)(void*, void*);
  void *(*get_text)(void*);
  void (*set_text)(void*, void*);
  void (*destroy)(void*);
} ext_button;

typedef struct {
  void (*deregister)(void*);
  void (*destroy)(void*);
} ext_handler_registration;

typedef struct {
  void *(*get_sub_texture)(void*, int64_t, int64_t, int64_t, int64_t);
  int64_t (*get_width)(void*);
  int64_t (*get_height)(void*);
  void (*destroy)(void*);
} ext_texture;

typedef struct {
  ext_main_menu_view main_menu_view;
  ext_game_view game_view;
  ext_button button;
  ext_handler_registration handler_registration;
  ext_texture texture;
} ext_ui_binding;

typedef struct {
  void *(*load_texture)(void*, void*);
  void (*destroy)(void*);
} ext_texture_loader;

typedef struct {
  ext_texture_loader texture_loader;
} ext_native_binding;

void *bind_game_view(ext_application_context application_context,
                                           void *ext_view);

void *bind_main_menu_view(ext_application_context application_context,
                                                        void *ext_view);

ext_application_context create_application_context(void *ext_texture_loader,
                                                   ext_ui_binding ext_ui,
                                                   ext_native_binding ext_native);

#endif /* enchantron_h */
