<div align="center">
  <h1><code>sss</code></h1>
  <p><strong>simple static sites</strong></p>
</div>

sss is a very simple static site generator, meant to give you only the
necessary functionality and staying out of your way as much as possible. It
has no configuration file, no dependencies and weighs in at only ~350 kB. 

# Directory structure

The default directory structure is as follows:

- `src`: This directory contains source files that directly map to a HTML file
in the output. They can be any filetype.

    You can override this directory by setting the environment variable 
    `SSS_SRC`.

- `tmpl`: This directory contains your templates. It has to include at least a
file called `default.html`, which will be the fallback template for all files
in the `src` directory. If a HTML file in this directory is the equivalent of a
source file in the source directory (e.g. `tmpl/subdir/title.html` is
equivalent to `src/subdir/title.md`), it will be used as a template for that
source.

    You can override this directory by setting the environment variable
    `SSS_TMPL`.

- `dst`: This directory will hold your generated site. If you want any static
files in your website, place them here.

    You can override this directory by setting the environment variable
    `SSS_DST`.
    
# Templating

Templating is done in POSIX shell, which allows great flexibility in how you
want your pages rendered. The templates have an extremely simple syntax. You
start in echo-mode, and alter between echo-mode and script-mode by placing `#!
` on a separate line. In echo-mode, regular shell string interpolation can be
used, while in script-mode, you can write in regular shell script.

Consider e.g. the following template:

```html
<ul>
#!
for f in range "file1 subdir/file2 sub/subsub/file3"; do
#!
<li>File: $f</li>
#!
done
</ul>
```

It will get rendered as:

```html
<ul>
<li>File: file1</li>
<li>File: subdir/file2</li>
<li>File: sub/subsub/file3</li>
</ul>
```

This means that including a file's content is as easy as `cat`ing it in the
template, and rendering markdown can be done by interpolating e.g. a Pandoc
command like `$(pandoc -f markdown -t html $src)`.

All templates have the following variables available to them:

- `$srcs`: An array of paths pointing to all source files. This array is
newline-separated, so you can handle filenames with spaces.
- `$src_dir$`: The path to the source directory.
- `$tmpl_dir$`: The path to the template directory.
- `$dst_dir$`: The path to the output directory.

Additionally, the templates that map to a source file (`default.html` and
templates matching a source path) also have the following variables available
to them:

- `$src`: The path pointing to the source file the template is currently
rendering.
- `$path_to_root`: A path to the root from the source file.
  
    Example: The source `src/subdir/subsubdir/file.md` will have
    `path_to_root="../.."`.
    
    Example: The source `src/file.md` will have `path_to_root=""`.
