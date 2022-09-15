<div align="center">
  <h1><code>sss</code></h1>
  <p><strong>simple static sites</strong></p>
</div>

sss is a very simple static site generator, meant to give you only the
necessary functionality and staying out of your way as much as possible. It
has no configuration file, no preferred structure and you decide on how you
want your rendering done. Templating is done in POSIX shell, and content/local
variables are passed as environment variables.

sss has no dependencies and is built together with the Rust standard library
from scratch. This ensures no redundant code is compiled to the final binary,
which weighs in at only ~300 kB (!!).
