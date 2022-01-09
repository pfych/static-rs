# static-rs
This is a rewrite of some of the functions used in [static-site](https://github.com/pfych/Static) in `Rust`.  
Originally they were Shell functions, however I had issues manipulating XML/HTML in Shell.

I used this as an excuse to learn `Rust`.

## Usage
Create a `config.json` file:
```
{
  "blog_location": "/home/pfych/Documents/Scratchpad-write",
  "image_location": "/home/pfych/Documents/Scratchpad-write/images",
  "blog_template": "./template/blog/index.html",
  "index_template": "./template/index.html",
  "author": "pfych",
  "url": "https://pfy.ch",
  "file_suffix": "-write.md"
}
```

- `blog_location`: Where are the blog `.md` files  
- `image_location`: Where are images for the blog stored  
- `blog_template`: Pandoc html template for blogs  
- `index_template`: index html, Table of contents will be injected wherever the string `TOC` is.  
- `author`: Who should `rss` report the author as  
- `url`: What url will this run behind (for `rss`)  
- `file_suffix`: what does the blog file end with?

Running the binary with the `config.json` file at the same directory level should product an out folder with your sites "dynamic content".
