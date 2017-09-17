This generates a small Linux filesystem. In default config needs 
UPX, linux headers, Rust toolchain and GCC toolchain. Resulting
fs is a statically linked.

---

Tento program generuje Linuxovou adresářovou strukturu podle
[designu](https://sta.li/filesystem) distribuce stali skupiny suckless.org.

### Obsah

0. [Požadavky](#požadavky)
0. [Konfigurace](#konfigurace)
0. [Programy](#programy)
0. [Adresářová struktura](#adresářová-struktura)
0. [Průběh](#průběh)
0. [Bezpečnost](#bezpečnost)

### Požadavky

* GCC toolchain
* Rust toolchain (doporučována nejnovější nightly verze)
* [UPX](https://upx.github.io/)
* linux, asm, asm-generic headery ve složce /usr/include (pouze při kompilaci programů z 9base)
* git
* curl
* make
* základní unixové programy (cp, rm, ln, sed, grep...)

### Konfigurace

Minilinuxfs lze konfigurovat pomocí souboru __cfg.toml__ v kořenovém
adresáři repozitáře. Program používá TOML pro konfiguraci.

- Cestu k adresáři s kostrou adresářové struktury a konfiguračnímy soubory lze
  změnit pomocí hodnoty __input__
- Cestu výstupního adresáře lze změnit pomocí hodnoty __output__
- Cestu k adresáři kde dochází ke stažení repozitářů a kompilaci programů lze
  změnit pomocí hodnoty __git__

Zbytek konfiguračního souboru jsou záznamy pro jednotlivé repozitáře s programy.
Každý záznam musí začínat řádkem obsahujícím text "[[entries]]". Následují parametry
ve volitelném pořadí:

| Název   | Popis                                        | Povinný          |
|---------|----------------------------------------------|------------------|
| url     | Adresa gitového repozitáře                   | Ano              |
| name    | Jméno, ovlivňuje pouze název složky uložení\*| Ano              |
| cmds    | Příkazy ke spuštění kompilaci                | Může být prázdný |
| paths   | Pole dvojic cest: zdroj a cíl\*\*            | Může být prázdný |
| install | Příkazy ke spuštení po kompilace             | Ne               |

\* = Může být absolutní cesta  
\*\* = zdroj je relativní ke kořenu daného repozitáře, cíl je relativní ke kořenu  
  výsledné adresářové struktury

Pozn 1: U pole install lze v příkazeh využít hodnotu '<\<target\>>', která reprezentuje
absolutní cestu k výslednému adresáři.

Pozn 2: Příkazy jak u __cmds__ tak u __install__ nejsou plnohodnotné shellové příkazy.
Začátek musí vždy být název platného spustitelného souboru, následovaný parametry pro
daný příkaz oddělenými libovolnými bílými znaky.

### Programy

V základní konfigurace stáhne MiniLinuxFS 4 repozitáře:
**sbase**, **ubase**, **9base** a **bash-static**. Z nich využívá jen některé programy:

| sbase                               | ubase                             | 9base                           |
|-------------------------------------|-----------------------------------|---------------------------------|
| <ul><li>[ ] basename</li></ul>      | <ul><li>[ ] chvt</li></ul>        | <ul><li>[ ] ascii</li></ul>     |
| <ul><li>[ ] cal</li></ul>           | <ul><li>[ ] clear</li></ul>       | <ul><li>[x] awk</li></ul>       |
| <ul><li>[x] cat</li></ul>           | <ul><li>[ ] ctrlaltdel</li></ul>  | <ul><li>[ ] basename</li></ul>  |
| <ul><li>[ ] chgrp</li></ul>         | <ul><li>[ ] dd</li></ul>          | <ul><li>[ ] bc</li></ul>        |
| <ul><li>[x] chmod</li></ul>         | <ul><li>[ ] df</li></ul>          | <ul><li>[ ] cal</li></ul>       |
| <ul><li>[ ] chown</li></ul>         | <ul><li>[ ] dmesg</li></ul>       | <ul><li>[ ] cat</li></ul>       |
| <ul><li>[ ] chroot</li></ul>        | <ul><li>[ ] eject</li></ul>       | <ul><li>[ ] cleanname</li></ul> |
| <ul><li>[ ] cksum</li></ul>         | <ul><li>[ ] fallocate</li></ul>   | <ul><li>[ ] cmp</li></ul>       |
| <ul><li>[x] cmp</li></ul>           | <ul><li>[ ] free</li></ul>        | <ul><li>[ ] date</li></ul>      |
| <ul><li>[ ] cols</li></ul>          | <ul><li>[ ] freeramdisk</li></ul> | <ul><li>[ ] dc</li></ul>        |
| <ul><li>[ ] comm</li></ul>          | <ul><li>[ ] fsfreeze</li></ul>    | <ul><li>[ ] dd</li></ul>        |
| <ul><li>[x] cp</li></ul>            | <ul><li>[ ] getty</li></ul>       | <ul><li>[x] diff</li></ul>      |
| <ul><li>[ ] cron</li></ul>          | <ul><li>[ ] halt</li></ul>        | <ul><li>[ ] du</li></ul>        |
| <ul><li>[ ] cut</li></ul>           | <ul><li>[ ] hwclock</li></ul>     | <ul><li>[ ] echo</li></ul>      |
| <ul><li>[x] date</li></ul>          | <ul><li>[ ] id</li></ul>          | <ul><li>[ ] ed</li></ul>        |
| <ul><li>[ ] dirname</li></ul>       | <ul><li>[ ] insmod</li></ul>      | <ul><li>[ ] factor</li></ul>    |
| <ul><li>[ ] du</li></ul>            | <ul><li>[ ] killall</li></ul>     | <ul><li>[ ] fmt</li></ul>       |
| <ul><li>[x] echo</li></ul>          | <ul><li>[ ] last</li></ul>        | <ul><li>[ ] fortune</li></ul>   |
| <ul><li>[ ] ed</li></ul>            | <ul><li>[ ] lastlog</li></ul>     | <ul><li>[ ] freq</li></ul>      |
| <ul><li>[ ] env</li></ul>           | <ul><li>[ ] login</li></ul>       | <ul><li>[ ] getflags</li></ul>  |
| <ul><li>[ ] expand</li></ul>        | <ul><li>[ ] lsmod</li></ul>       | <ul><li>[ ] grep</li></ul>      |
| <ul><li>[ ] expr</li></ul>          | <ul><li>[ ] lsusb</li></ul>       | <ul><li>[ ] hoc</li></ul>       |
| <ul><li>[x] false</li></ul>         | <ul><li>[ ] mesg</li></ul>        | <ul><li>[ ] join</li></ul>      |
| <ul><li>[x] find</li></ul>          | <ul><li>[x] mknod</li></ul>       | <ul><li>[ ] listen1</li></ul>   |
|                                     | <ul><li>[ ] mkswap</li></ul>      | <ul><li>[ ] look</li></ul>      |
| <ul><li>[ ] flock</li></ul>         | <ul><li>[ ] mount</li></ul>       | <ul><li>[ ] ls</li></ul>        |
| <ul><li>[ ] fold</li></ul>          | <ul><li>[ ] mountpoint</li></ul>  | <ul><li>[ ] md5sum</li></ul>    |
| <ul><li>[ ] getconf</li></ul>       | <ul><li>[ ] nologin</li></ul>     | <ul><li>[ ] mk</li></ul>        |
| <ul><li>[x] grep</li></ul>          | <ul><li>[ ] pagesize</li></ul>    | <ul><li>[ ] mkdir</li></ul>     |
| <ul><li>[x] head</li></ul>          | <ul><li>[ ] passwd</li></ul>      | <ul><li>[ ] mtime</li></ul>     |
| <ul><li>[ ] hostname</li></ul>      | <ul><li>[ ] pidof</li></ul>       | <ul><li>[ ] pbd</li></ul>       |
| <ul><li>[ ] join</li></ul>          | <ul><li>[ ] pivot_root</li></ul>  |                                 |
| <ul><li>[ ] kill</li></ul>          | <ul><li>[ ] ps</li></ul>          | <ul><li>[ ] primes</li></ul>    |
| <ul><li>[ ] link</li></ul>          | <ul><li>[ ] pwdx</li></ul>        | <ul><li>[ ] rc</li></ul>        |
| <ul><li>[x] ln</li></ul>            | <ul><li>[ ] readahead</li></ul>   | <ul><li>[ ] read</li></ul>      |
| <ul><li>[ ] logger</li></ul>        | <ul><li>[ ] respawn</li></ul>     | <ul><li>[ ] rm</li></ul>        |
| <ul><li>[ ] logname</li></ul>       | <ul><li>[ ] rmmod</li></ul>       | <ul><li>[ ] sam</li></ul>       |
| <ul><li>[x] ls</li></ul>            | <ul><li>[ ] stat</li></ul>        | <ul><li>[ ] sed</li></ul>       |
| <ul><li>[ ] md5sum</li></ul>        | <ul><li>[ ] su</li></ul>          | <ul><li>[ ] seq</li></ul>       |
| <ul><li>[x] mkdir</li></ul>         | <ul><li>[ ] swaplabel</li></ul>   | <ul><li>[ ] sha1sum</li></ul>   |
| <ul><li>[ ] mkfifo</li></ul>        | <ul><li>[ ] swapoff</li></ul>     | <ul><li>[ ] sleep</li></ul>     |
| <ul><li>[ ] mktemp</li></ul>        | <ul><li>[ ] swapon</li></ul>      | <ul><li>[ ] sort</li></ul>      |
| <ul><li>[x] mv</li></ul>            | <ul><li>[ ] switch_root</li></ul> | <ul><li>[ ] split</li></ul>     |
| <ul><li>[ ] nice</li></ul>          | <ul><li>[ ] sysctl</li></ul>      | <ul><li>[ ] ssam</li></ul>      |
| <ul><li>[x] nl</li></ul>            | <ul><li>[ ] truncate</li></ul>    | <ul><li>[ ] strings</li></ul>   |
| <ul><li>[ ] nohup</li></ul>         | <ul><li>[ ] umount</li></ul>      | <ul><li>[ ] tail</li></ul>      |
| <ul><li>[ ] od</li></ul>            | <ul><li>[ ] unshare</li></ul>     | <ul><li>[ ] tee</li></ul>       |
| <ul><li>[ ] paste</li></ul>         | <ul><li>[ ] uptime</li></ul>      | <ul><li>[ ] test</li></ul>      |
| <ul><li>[ ] pathchk</li></ul>       | <ul><li>[ ] vtallow</li></ul>     | <ul><li>[ ] touch</li></ul>     |
| <ul><li>[ ] printenv</li></ul>      | <ul><li>[ ] watch</li></ul>       | <ul><li>[ ] tr</li></ul>        |
| <ul><li>[ ] printf</li></ul>        | <ul><li>[ ] who</li></ul>         | <ul><li>[ ] troff</li></ul>     |
| <ul><li>[ ] pwd</li></ul>           |                                   | <ul><li>[ ] unicode</li></ul>   |
| <ul><li>[ ] readlink</li></ul>      |                                   | <ul><li>[ ] uniq</li></ul>      |
| <ul><li>[ ] renice</li></ul>        |                                   | <ul><li>[ ] unutf</li></ul>     |
| <ul><li>[x] rev</li></ul>           |                                   | <ul><li>[ ] urlencode</li></ul> |
| <ul><li>[x] rm</li></ul>            |                                   | <ul><li>[ ] wc</li></ul>        |
| <ul><li>[x] rmdir</li></ul>         |                                   | <ul><li>[ ] yacc</li></ul>      |
| <ul><li>[x] sed</li></ul>           |                                   |                                 |
| <ul><li>[x] seq</li></ul>           |                                   |                                 |
| <ul><li>[ ] setsid</li></ul>        |                                   |                                 |
| <ul><li>[ ] sha1sum</li></ul>       |                                   |                                 |
| <ul><li>[ ] sha224sum</li></ul>     |                                   |                                 |
| <ul><li>[ ] sha256sum</li></ul>     |                                   |                                 |
| <ul><li>[ ] sha384sum</li></ul>     |                                   |                                 |
| <ul><li>[ ] sha512</li></ul>-224sum |                                   |                                 |
| <ul><li>[ ] sha512</li></ul>-256sum |                                   |                                 |
| <ul><li>[ ] sha512sum</li></ul>     |                                   |                                 |
| <ul><li>[x] sleep</li></ul>         |                                   |                                 |
| <ul><li>[x] sort</li></ul>          |                                   |                                 |
| <ul><li>[ ] split</li></ul>         |                                   |                                 |
| <ul><li>[ ] sponge</li></ul>        |                                   |                                 |
| <ul><li>[ ] strings</li></ul>       |                                   |                                 |
| <ul><li>[ ] sync</li></ul>          |                                   |                                 |
| <ul><li>[x] tail</li></ul>          |                                   |                                 |
| <ul><li>[x] tar</li></ul>           |                                   |                                 |
| <ul><li>[x] tee</li></ul>           |                                   |                                 |
| <ul><li>[ ] test</li></ul>          |                                   |                                 |
| <ul><li>[ ] tftp</li></ul>          |                                   |                                 |
| <ul><li>[ ] time</li></ul>          |                                   |                                 |
| <ul><li>[x] touch</li></ul>         |                                   |                                 |
| <ul><li>[x] tr</li></ul>            |                                   |                                 |
| <ul><li>[x] true</li></ul>          |                                   |                                 |
| <ul><li>[ ] tsort</li></ul>         |                                   |                                 |
| <ul><li>[ ] tty</li></ul>           |                                   |                                 |
| <ul><li>[ ] uname</li></ul>         |                                   |                                 |
| <ul><li>[ ] unexpand</li></ul>      |                                   |                                 |
| <ul><li>[x] uniq</li></ul>          |                                   |                                 |
| <ul><li>[ ] unlink</li></ul>        |                                   |                                 |
| <ul><li>[ ] uudecode</li></ul>      |                                   |                                 |
| <ul><li>[ ] uuencode</li></ul>      |                                   |                                 |
| <ul><li>[x] wc</li></ul>            |                                   |                                 |
| <ul><li>[ ] which</li></ul>         |                                   |                                 |
| <ul><li>[x] whoami</li></ul>        |                                   |                                 |
| <ul><li>[x] xargs</li></ul>         |                                   |                                 |
| <ul><li>[ ] xinstall</li></ul>      |                                   |                                 |
| <ul><li>[ ] yes</li></ul>           |                                   |                                 |


A samozřejmě bash z bash-static. U 9base je potřeba další 
programy přidat i v Makefile-u. Bash a programy z sbase a ubase
jsou komprimovány pomocí UPX. Velikost bashe je také snížená pomocí
programu strip.

Všechny programy jsou staticky linkované a používají musl libc.

### Adresářová struktura

MiniLinuxFS v základní konfiguraci využívá adresářovou strukturu sta.li:

> / - the root home  
> /bin - all executables  
> /sbin -> /bin # softlink pointing to /bin  
> /boot - all boot files  
> /etc - system configuration  
> /home - user directories  
> /var - spool, run, log, cache  
> /share - man pages, locales, dependencies  
> /include - include/headers  
> /lib - static libraries for building stuff  
> /mnt - mount points  
> /usr -> / # softlink pointing to /  
> /dev - devices  
> /proc - proc files  
> /sys - sys files  

### Průběh

__Normální průběh je vyvolán příkazem 'make'.__

1. Po spuštění příkazu cargo postaví program install
0. Make zkopíruje install do kořenového adresáře repozitáře
0. install se pokousí přečíst a parsovat konfigurační soubor.
   V případě neúspěchu, program se ukončí s kódem -1, když se
   nepodaří soubor přečíst a -2 když se jej nepodaří parsovat.
0. install postupně zklonuje všechny repozitáře, při chybě se
   ukončí s kódem -3
0. install postupně spouští všechny příkazy pro kompilaci programů,
   pokud některý z programů vrátí nenulovou hodnotu, ukončí se
   s kódem -4
0. install zkopíruje všechny soubory specifikované v poli __paths__
   jednotlivých záznamů, pokud nelze soubor zkopírovat, program panikaří
0. install spustí všechny příkazy v poli __install__

Pozn 1: V případě chyby v kopírování, změně/přečtení CWD nebo neúspěchu spustit příkaz
kód panikaří

Pozn 2: Kopírování vstupní souborové struktury používá _'cp -a'_ místo funkce kvůli
zachování symbolických odkazů.

Pozn 3: Při znovu sestavení install maže a znovu-klonuje původní repozitáře.
Neměly by tedy proto obsahovat žádné lokální změny mimo soubory vytvořené kompilací

Pozn 4: Výsledky mazání souborů jsou zahozeny

__Resetovací průběh je zpuštěn příkazem 'make clean'__

1. install je spuštěn s parametrem 'clean'
0. Pokusí se přečíst a parsovat konfigurační soubor
0. V případě neúspěchu se vypne, jinak smaže výstupní složku
   a složku se staženými repozitáři programů.
0. Make (resp. rm) smaže složku s artifakty programu install
   a program samotný

Pozn: Make ignoruje chybu pokud byl ./install již dříve smazán

__Program byl otestován na distribuci Arch s triplety x86\_64-unkown-linux-gnu__
__a armv7l-unkown-linux-gnueabihf__

### Bezpečnost

V základní konfiguraci produkuje MiniLinuxFS relativně bezpečné
prostředí:

* Všechny programy jsou staticky linkované -> není nutnost připojit
  složky /lib a /lib64 hostitelského systému
* K normálnímu běhu základních programů není nutné připojit ani /proc,
  ani /dev
* Základní programy neobsahují nic co by mohlo zobrazit procesy hostitelského
  systému (ps, pstree, top), změnit uživatele (su, sudo) nebo ovlivnit
  stav stroje (halt, shutdown, reboot)
* Nepřítomnost kompilátorů (např pro C/C++ nebo Perl) zamezuje využítí
  exploitů
* Neobsahuje programy které by vyžadovaly /sys

Chroot jail tedy může být kompletně izolovaný od okolního systému
a útect z něj je velmi obtížné, ne-li nemožné, bez ohledu na to, jestli
je uživatel root.
