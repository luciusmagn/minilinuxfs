This generates a small Linux filesystem. In default config needs 
UPX, linux headers, Rust toolchain and GCC toolchain. Resulting
fs is a statically linked.

---

Tento program generuje Linuxovou adresářovou strukturu podle
[designu](https://sta.li/filesystem) distribuce stali skupiny suckless.org.

### Obsah

0. [Požadavky](#Požadavky)
0. [Konfigurace](#Konfigurace)
0. [Programy](#Programy)
0. [Adresářová struktura](#Adresářová-struktura)
0. [Průběh](#Průběh)
0. [Bezpečnost](#Bezpečnost)

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

| sbase            | ubase          | 9base        |
|------------------|----------------|--------------|
| [ ] basename     | [ ] chvt       | [ ] ascii    |
| [ ] cal          | [ ] clear      | [x] awk      |
| [x] cat          | [ ] ctrlaltdel | [ ] basename |
| [ ] chgrp        | [ ] dd         | [ ] bc       |
| [x] chmod        | [ ] df         | [ ] cal      |
| [ ] chown        | [ ] dmesg      | [ ] cat      |
| [ ] chroot       | [ ] eject      | [ ] cleanname|
| [ ] cksum        | [ ] fallocate  | [ ] cmp      |
| [x] cmp          | [ ] free       | [ ] date     |
| [ ] cols         | [ ] freeramdisk| [ ] dc       |
| [ ] comm         | [ ] fsfreeze   | [ ] dd       |
| [x] cp           | [ ] getty      | [x] diff     |
| [ ] cron         | [ ] halt       | [ ] du       |
| [ ] cut          | [ ] hwclock    | [ ] echo     |
| [x] date         | [ ] id         | [ ] ed       |
| [ ] dirname      | [ ] insmod     | [ ] factor   |
| [ ] du           | [ ] killall    | [ ] fmt      |
| [x] echo         | [ ] last       | [ ] fortune  |
| [ ] ed           | [ ] lastlog    | [ ] freq     |
| [ ] env          | [ ] login      | [ ] getflags |
| [ ] expand       | [ ] lsmod      | [ ] grep     |
| [ ] expr         | [ ] lsusb      | [ ] hoc      |
| [x] false        | [ ] mesg       | [ ] join     |
| [x] find         | [x] mknod      | [ ] listen1  |
|                  | [ ] mkswap     | [ ] look     |
| [ ] flock        | [ ] mount      | [ ] ls       |
| [ ] fold         | [ ] mountpoint | [ ] md5sum   |
| [ ] getconf      | [ ] nologin    | [ ] mk       |
| [x] grep         | [ ] pagesize   | [ ] mkdir    |
| [x] head         | [ ] passwd     | [ ] mtime    |
| [ ] hostname     | [ ] pidof      | [ ] pbd      |
| [ ] join         | [ ] pivot_root |              |
| [ ] kill         | [ ] ps         | [ ] primes   |
| [ ] link         | [ ] pwdx       | [ ] rc       |
| [x] ln           | [ ] readahead  | [ ] read     |
| [ ] logger       | [ ] respawn    | [ ] rm       |
| [ ] logname      | [ ] rmmod      | [ ] sam      |
| [x] ls           | [ ] stat       | [ ] sed      |
| [ ] md5sum       | [ ] su         | [ ] seq      |
| [x] mkdir        | [ ] swaplabel  | [ ] sha1sum  |
| [ ] mkfifo       | [ ] swapoff    | [ ] sleep    |
| [ ] mktemp       | [ ] swapon     | [ ] sort     |
| [x] mv           | [ ] switch_root| [ ] split    |
| [ ] nice         | [ ] sysctl     | [ ] ssam     |
| [x] nl           | [ ] truncate   | [ ] strings  |
| [ ] nohup        | [ ] umount     | [ ] tail     |
| [ ] od           | [ ] unshare    | [ ] tee      |
| [ ] paste        | [ ] uptime     | [ ] test     |
| [ ] pathchk      | [ ] vtallow    | [ ] touch    |
| [ ] printenv     | [ ] watch      | [ ] tr       |
| [ ] printf       | [ ] who        | [ ] troff    |
| [ ] pwd          |                | [ ] unicode  |
| [ ] readlink     |                | [ ] uniq     |
| [ ] renice       |                | [ ] unutf    |
| [x] rev          |                | [ ] urlencode|
| [x] rm           |                | [ ] wc       |
| [x] rmdir        |                | [ ] yacc     |
| [x] sed          |                |              |
| [x] seq          |                |              |
| [ ] setsid       |                |              |
| [ ] sha1sum      |                |              |
| [ ] sha224sum    |                |              |
| [ ] sha256sum    |                |              |
| [ ] sha384sum    |                |              |
| [ ] sha512-224sum|                |              |
| [ ] sha512-256sum|                |              |
| [ ] sha512sum    |                |              |
| [x] sleep        |                |              |
| [x] sort         |                |              |
| [ ] split        |                |              |
| [ ] sponge       |                |              |
| [ ] strings      |                |              |
| [ ] sync         |                |              |
| [x] tail         |                |              |
| [x] tar          |                |              |
| [x] tee          |                |              |
| [ ] test         |                |              |
| [ ] tftp         |                |              |
| [ ] time         |                |              |
| [x] touch        |                |              |
| [x] tr           |                |              |
| [x] true         |                |              |
| [ ] tsort        |                |              |
| [ ] tty          |                |              |
| [ ] uname        |                |              |
| [ ] unexpand     |                |              |
| [x] uniq         |                |              |
| [ ] unlink       |                |              |
| [ ] uudecode     |                |              |
| [ ] uuencode     |                |              |
| [x] wc           |                |              |
| [ ] which        |                |              |
| [x] whoami       |                |              |
| [x] xargs        |                |              |
| [ ] xinstall     |                |              |
| [ ] yes          |                |              |

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
