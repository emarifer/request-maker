subdir('icons')

blueprint_files = [
  'ui/main_window.blp',
  'ui/request_header_pane.blp',
  'ui/request_header_row.blp',
  'ui/response_panel.blp',
]

blueprint_targets = []
foreach blp : blueprint_files
  blueprint_targets += blp.replace('.blp', '.ui').split('/')[-1]
endforeach

blueprints = custom_target('blueprints',
  input: files(blueprint_files),
  output: blueprint_targets,
  command: [find_program('blueprint-compiler'), 'batch-compile', '@OUTDIR@', '@CURRENT_SOURCE_DIR@', '@INPUT@'],
)

resources = gnome.compile_resources(
  'request-maker',
  'request-maker.gresource.xml',
  dependencies: blueprints,
  gresource_bundle: true,
  source_dir: meson.current_build_dir(),
  install: true,
  install_dir: pkgdatadir,
)

desktop_file = configure_file(
  input: 'request-maker.desktop.in',
  output: '@0@.desktop'.format(application_id),
  configuration: {'app_id': application_id},
  install_dir: join_paths(get_option('datadir'), 'applications'),
)

desktop_utils = find_program('desktop-file-validate', required: true)
test('Validate desktop file', desktop_utils, args: [desktop_file])
