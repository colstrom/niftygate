/^#.+$/d
s/log::info[!]/console.log/
s/pub[(]crate[)] fn/function/
s/vec![[]0 as u8; ([^]]+)[]]/new ArrayBuffer(\1)/
s/#if ([^ ]+) == 0 [{] ([^}]+) [}] else [{] ([^}]+) [}]/\1 || \2/
s/ rs / js /g
s/\"/\`/g
s/({[^{}]+})/$\1/g
s/ [{][}]//g
s/: [^=]+=/ =/
s/: [^,]+,/,/g
s/: [^)]+[)]/\)/
s/-> [^{]+{/\{/
s/let( mut)?/var/
s/ as [ui](size|64)//
s/[&]mut//
s/[.]len[(][)]/.length/
s/[(]z[)]/z/
s/loop [{]/while (true) {/