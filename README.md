# metaforge - v0.1.2

a pattern driven static site generator for extensible snippet insertion.

### dependencies

- cargo
- pandoc (reccommended)

### installing:

    $ cd metaforge
    $ cargo install --path .

## about

metaforge is a static site generator that lets you write something once, and re-use it
across your site. it requires previous knowledge of html, and doesn't come with pre-made
themes or templates except for a completely bare skeleton directory. it gives you extremely
fine grained control over the generated html, customizing each pattern to a source file
with variables and mappable arrays.

metaforge also lets you write metapatterns that contain classes, masking choices to a
sub-set of patterns, and defining a fallback default pattern. combining this with variables
and arrays allow you to write completely customized and free form html that remains consistant
across your entire site, while still getting access to page level tinkering.

substitution and expansion is entirely disconnected from translation, which uses pandoc under the
hood. this means metaforge can be ran without pandoc, simply parsing and expanding files as needed.
this can be set on both a site and file level, allowing single files in the site to be simply expanded,
such as an rss feed. metaforge can also technically translate between any two document formats pandoc supports,
but nothing other than the default markdown to html gets tested.

the full documentation is available in this repository under the **files/README/source/docs**
directory. it's currently setup as a working example of a metaforge site, so you can poke around
and see one way a site can be setup.

### building documentation

    $ cargo test -- --ignored readme

this command can be run as many times as needed to regenerate the documentation, and is
reccomended after upgrading to a new version to see what's changed. the generated docs will
be available in **files/README/build**, and can be looked at in any web browser.
