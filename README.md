# Übungsblatt 3
## Allgemeine Hinweise
Für diese und alle folgenden Praktikumsaufgaben gilt, dass Einsendungen, die in der jeweils mitgegebenen Testumgebung nicht laufen, Punktabzug erhalten! 
Das beinhaltet insbesondere alle Programme, die sich nicht fehlerfrei kompilieren lassen. 
Da Cargo für die Ausführung verantwortlich ist, sollte das Projekt bei Ihnen am Ende mit `cargo test` ohne Fehler und Warnungen durchlaufen.

## Abgabemodus
Die Lösung ist in einem eigenen Git-Repository abzugeben.
Sie können in ihrer Lösung beliebige Hilfstypen und Module selbst definieren, jedoch dürfen die vorhandenen Testfälle im [tests](tests)-Ordner nicht abgeändert werden.

Zur Lösung der Aufgaben steht für Sie dieses Repository mit
- einem erweiterten [C1Lexer](src/lexer.rs)
- und einem Integrationstest in [parser.rs](tests/parser.rs) 

zur Verfügung.
In [parser.rs](src/parser.rs) befinden sich ein auskommentiertes Parser-Skelett mit Hilfsfunktionen und Testfällen. Sie können den auskommentierten Code nach Belieben nutzen oder löschen. 
> Sie können die Implementierung mit `cargo test` prüfen. Mit `cargo test -- --nocapture` werden Konsolenausgaben auch bei korrekten Tests angezeigt.


## Aufgabe 1 (100 Punkte)
### Kurzbeschreibung
Implementieren Sie von Hand einen Parser, der die Sprache C(-1) (eine Teilsprache von C1) erkennen kann. Verwenden Sie dazu wahlweise ihren Lexer aus Praktikumsaufgabe 2 oder den beigelegten, erweiterten [C1Lexer](src/lexer.rs). 

### Aufgabenstellung
Nachdem Sie in der letzten Praktikumsaufgabe einen Scanner für die lexikalische Analyse gebaut haben, sollen Sie sich diesmal mit der syntaktischen Analyse beschäftigen. Wie Sie bereits in der Vorlesung gelernt haben, gibt es mehrere Ansätze, einen Parser zu bauen. Insbesondere wird dabei zwischen handgeschriebenen Parsern und (durch Parsergeneratoren) generierten Parsern unterschieden.

In dieser Aufgabe soll ein handgeschriebener Parser nach dem Prinzip des [rekursiven Abstiegs](https://en.wikipedia.org/wiki/Recursive_descent_parser) implementiert werden. Da ein handgeschriebener Parser in der Regel ziemlich umfangreich ist, haben wir uns dazu entschlossen, die Sprache C1 (die ja durch Abrüsten aus C entstanden ist) noch einmal zu vereinfachen, um Ihnen extreme Tipporgien zu ersparen.
Des Weiteren sollten Sie sich mit den Methoden des [C1Lexer](src/lexer.rs) vertraut machen, da diese die Implementierung vereinfachen.

Sie finden die Grammatik von C(-1) [hier](https://amor.cms.hu-berlin.de/~kunert/lehre/material/c-1-grammar.php) und nachfolgend:
```C
program             ::= ( functiondefinition )* <EOF>

functiondefinition  ::= type <ID> "(" ")" "{" statementlist "}"
functioncall        ::= <ID> "(" ")"

statementlist       ::= ( block )*
block               ::= "{" statementlist "}"
                      | statement
statement           ::= ifstatement
                      | returnstatement ";"
                      | printf ";"
                      | statassignment ";"
                      | functioncall ";"

ifstatement         ::= <KW_IF> "(" assignment ")" block
returnstatement     ::= <KW_RETURN> ( assignment )?

printf              ::= <KW_PRINTF> "(" assignment ")"
type                ::= <KW_BOOLEAN>
                      | <KW_FLOAT>
                      | <KW_INT>
                      | <KW_VOID>

statassignment      ::= <ID> "=" assignment
assignment          ::= ( ( <ID> "=" assignment ) | expr )
expr                ::= simpexpr ( ( "==" | "!=" | "<=" | ">=" | "<" | ">" ) simpexpr )?
simpexpr            ::= ( "-" )? term ( ( "+" | "-" | "||" ) term )*
term                ::= factor ( ( "*" | "/" | "&&" ) factor )*
factor              ::= <CONST_INT>
                      | <CONST_FLOAT>
                      | <CONST_BOOLEAN>
                      | functioncall
                      | <ID>
                      | "(" assignment ")"
```
Zusätzlich sind folgende Punkte zu beachten:

- die Implementation kann in einem beliebigen Modul erfolgen, jedoch muss ein Typ `C1Parser` über re-export in lib.rs nach außen sichtbar gemacht werden. 
- für `C1Parser` muss es eine _associated function_ mit der Signatur `pub fn parse(text: &str) -> ParseResult` geben, die im Integrationstest aufgerufen werden kann.
- `parse` is lediglich dazu da, den übergebenen Text auf Syntaxfehler zu prüfen, ohne die geparsten Werte in irgendeiner Form zu speichern oder weiterzuverarbeiten. Dies sollte die Implementierung etwas erleichtern. 
- `parse` gibt im erfolgreichen Fall ein `Result::Ok` zurück
- bei einem Fehler wird ein `Result::Err` zurückgegeben, in dem eine Fehlermeldung mit der Fehlerstelle (Zeilennummer) eingebettet ist.   
- wenn Sie den Parser mithilfe des Integrationstests auf das mitgelieferte C(-1)-Beispielprogramm beispiel.c-1 ansetzen, sollte kein Fehler gemeldet werden - sehen Sie bitte diesen Test als Mindestvoraussetzung für eine Abgabe an.

### Weitere Hinweise
> Die folgenden Hinweise dienen als weitere Hilfestellung. Ihre Umsetzung ist keine Pflicht, aber erleichtert Ihnen möglicherweise die Implementierung.

- (Optional) Implementieren (und benutzen) Sie eine Methode/Funktion namens `eat()`, die das aktuelle Token konsumiert. Der Lexer stellt eine entsprechende Methode bereit, die wiederverwendet werden kann.
- (Optional) Implementieren (und benutzen) Sie eine Funktion/Methode namens `check_and_eat_token(token: C1Token)`, die überprüft, ob das ihr übergebene Token gleich dem aktuellen ist. Im Positivfall wird das aktuelle Token konsumiert, im Negativfall wird ein Fehler zurückgegeben.
- (Optional) Implementieren (und benutzen) Sie zwei Funktionen/Methoden namens `current_matches(token: C1Token)` und `next_matches(token: C1Token)`, die überprüfen, ob das ihnen übergebene Token gleich dem aktuellen bzw. nächsten Token ist und das Ergebnis des Vergleichs zurückgibt.

Viel Erfolg bei der Bearbeitung!
