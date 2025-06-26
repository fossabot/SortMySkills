use std::io::{self, Write};

fn main() {
    let icons_string = "ableton,activitypub,actix,adonis,ae,aiscript,alpinejs,anaconda,androidstudio,angular,ansible,apollo,apple,appwrite,arch,arduino,astro,atom,au,autocad,aws,azul,azure,babel,bash,bevy,bitbucket,blender,bootstrap,bsd,bun,c,cs,cpp,crystal,cassandra,clion,clojure,cloudflare,cmake,codepen,coffeescript,css,cypress,d3,dart,debian,deno,devto,discord,bots,discordjs,django,docker,dotnet,dynamodb,eclipse,elasticsearch,electron,elixir,elysia,emacs,ember,emotion,express,fastapi,fediverse,figma,firebase,flask,flutter,forth,fortran,gamemakerstudio,gatsby,gcp,git,github,githubactions,gitlab,gmail,gherkin,go,gradle,godot,grafana,graphql,gtk,gulp,haskell,haxe,haxeflixel,heroku,hibernate,html,htmx,idea,ai,instagram,ipfs,java,js,jenkins,jest,jquery,kafka,kali,kotlin,ktor,kubernetes,laravel,latex,less,linkedin,linux,lit,lua,md,mastodon,materialui,matlab,maven,mint,misskey,mongodb,mysql,neovim,nestjs,netlify,nextjs,nginx,nim,nix,nodejs,notion,npm,nuxtjs,obsidian,ocaml,octave,opencv,openshift,openstack,p5js,perl,ps,php,phpstorm,pinia,pkl,plan9,planetscale,pnpm,postgres,postman,powershell,pr,prisma,processing,prometheus,pug,pycharm,py,pytorch,qt,r,rabbitmq,rails,raspberrypi,react,reactivex,redhat,redis,redux,regex,remix,replit,rider,robloxstudio,rocket,rollupjs,ros,ruby,rust,sass,spring,sqlite,stackoverflow,styledcomponents,sublime,supabase,scala,sklearn,selenium,sentry,sequelize,sketchup,solidity,solidjs,svelte,svg,swift,symfony,tailwind,tauri,tensorflow,terraform,threejs,twitter,ts,ubuntu,unity,unreal,v,vala,vercel,vim,visualstudio,vite,vitest,vscode,vscodium,vue,vuetify,wasm,webflow,webpack,webstorm,windicss,windows,wordpress,workers,xd,yarn,yew,zig";

    print!("Paste your icon list: ");
    io::stdout().flush().unwrap();

    let mut icons = String::new();
    io::stdin().read_line(&mut icons).unwrap();
    icons = icons.trim().to_lowercase();

    if icons.is_empty() {
        icons = icons_string.parse().unwrap();
        println!("Emptying the list and using the provided icons.");
    }
    
    let mut remaining: Vec<&str> = icons.split(',').collect();
    let mut selected = Vec::new();

    loop {
        println!("\nRemaining:");
        println!("{}", remaining.join(","));

        print!("Select an icon (name, or Enter to finish): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        choice = choice.trim().to_lowercase();

        if choice.is_empty() {
            break;
        }

        let mut choices: Vec<&str> = choice.split(',').collect();

        for choice in &mut choices {
            if let Some(pos) = remaining.iter().position(|&icon| icon == *choice) {
                remaining.remove(pos);
                selected.push(choice.to_string());
            } else {
                println!("Not found: {}", choice);
            }
        }
    }


    if selected.len() == 0 {
        println!("No icons selected.");
        return;
    }


    println!("\nFinal Selection:");
    println!("{}", selected.join(","));

    println!("\nRemaining:");
    println!("{}", remaining.join(","));
}
