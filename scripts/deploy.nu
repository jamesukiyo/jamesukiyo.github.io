#!/usr/bin/env nu

def main [] {
   print "Building TailwindCSS..."
   bunx tailwindcss -i='./nerd/input.css' -o='./nerd/gen-tailwind.css' --minify --content='nerd/**/*.rs'
   bunx tailwindcss -i='./normal/input.css' -o='./normal/gen-tailwind.css' --minify --content='normal/**/*.rs'

   print "Running checks..."
   cargo check --quiet
   dx check --package nerd
   dx check --package normal

   # Build and deploy both versions.
   # Nerd must be last to prevent the normal deployment from deleting the sub-directory.
   for p in ["normal", "nerd"] {
      print $"Building ($p) version..."
      dx build --release --package $p
      mkdir $"($p)_dist"
      cp --recursive target/dx/($p)/release/web/public/* ($p)_dist/
      cp ($p)_dist/index.html ($p)_dist/404.html

      let dir = if $p == "nerd" { "nerd/" } else { "" }
      print $"Deploying ($p) version..."
      try {
         rsync -avz --delete ($p)_dist/ root@plum.taild29fec.ts.net:/var/www/site/($dir)
      } catch {|e|
         print --stderr $"Error: Deployment failed: ($e.msg)"
         exit 1
      }

      rm --recursive --force ($p)_dist
   }

   print "Deployment complete!"
}
