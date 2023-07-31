<!-- markdownlint-disable MD033 MD041 -->

# ddlc quick ending tas

<!-- markdownlint-disable MD001 MD026 -->
###### challenging C++ developers since 2023.
<!-- seriously, if you make super optimized code, try to beat me in time :skull: -->

<!-- markdownlint-enable MD001 MD026 -->
<!-- todo: use profiles -->

crappy rust TAS for the [quick ending](https://doki-doki-literature-club.fandom.com/wiki/Possible_Endings#Quick_Ending) for [Doki Doki Literature Club](https://ddlc.moe).

## [demo, and my own run of this game with this TAS.](https://youtu.be/1UQSrYjjNHs)

> Maybe you don't know what DDLC is, you should play it at nights when you just want to burn time ;)

I don't have ddlc plus, but this code *should* work the same.

## Features

- It works out of the box, just put it where DDLC is located.

- Slightly faster execution time than batch or shell.

## Getting started (user)

1: download ddlc quick ending tas.

2: put it in ddlc's directory, if you don't have the game, [download it](https://ddlc.moe).

3: run it.

4: make sure to record it and upload it to [youtube.com](https://youtube.com) and get 9999999999 subscribers for your TAS.

> For best time, it is recommended to run the game on a **HIGH END** machine, where the game is on your **SSD** or a **RAMDISK**.

## Getting started (developer)

1: Obtain a rust compiler, ``rustc`` will work fine, but i've used cargo. Cargo is usually included by default.

2: Make your changes.

3: ``cargo build --release`` or run ``build.bat``

4: Make sure to record your new and improved TAS and upload it to [youtube.com](https://youtube.com) and get 9999999999 subscribers for **your** TAS.

## Credits

[g](https://github.com/fikinoob) - Introducing me to Rust, and **being a gigachad**.

[Rust Foundation](https://github.com/rust-lang/) - Maintaining Rust even to today, even though this language was just a side project of a Mozilla employee.

## [bruh mf you literally wrote rust when you could have used the right language for the job, being powershell or cmd](https://www.youtube.com/watch?v=1UQSrYjjNHs&lc=UgyLryxrS2Bm2rcCBtt4AaABAg)

ok, heres the code in this repo, but its in batch:

```bat
start ddlc.exe&cd characters&ren sayori.chr s&WMIC.exe process where name='DDLC.exe' CALL setpriority realtime
```

very impressive right? reminds me of the smallest ``.bf`` compiler.

## ok but this virus because it deletes a file

**punjabi no virus**!

review code yourself && ``rustc main.rs`` it yourself.
