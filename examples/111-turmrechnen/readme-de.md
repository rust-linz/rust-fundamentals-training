# Turmrechnen

## Anforderung

Ihre Aufgabe ist das Schreiben einer Kommandozeilenanwendung in Rust zum [*Turmrechnen*](http://www.floriangeier.at/schule/kopf/kopf.php).

Ein Benutzer übergibt Ihrer Anwendung zwei Parameter über die Kommandozeile:

* Den Startwert (z.B. *9*); der Startwert muss > 1 sein.
* Die "Höhe" (z.B. *5); die Höhe muss > 2 sein.

Sie geben das Ergebnis in folgendem Format aus:

```txt
   9 * 2 = 18
  18 * 3 = 54
  54 * 4 = 216
 216 * 5 = 1080
1080 / 2 = 540
 540 / 3 = 180
 180 / 4 = 45
  45 / 5 = 9
```

Sollte der Benutzer falsche Parameter in der Kommandozeile eingegeben haben oder Parameter fehlen, geben Sie eine entsprechende Fehlermeldung am Bildschirm aus.

*Overflows* können Sie ignorieren.

## Tipps

* [Zugriff auf Kommandozeilenparameter](https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html)
* Sie können Ausgaben [links mit Leerzeichen auffüllen](https://doc.rust-lang.org/std/fmt/index.html#fillalignment)
* Sie können Strings in Integer mit [`parse`](https://doc.rust-lang.org/std/primitive.str.html#method.parse) umwandeln.

## Levels

Je nach Vorwissen wird diese Aufgabe manchen schwerer und manchen leichter fallen. Daher hier Anregungen, wie man Schritt für Schritt das Beispiel lösen könnte. Jeder kann basierend auf der bestehenden Programmierpraxis die Anzahl an Levels abarbeiten, die für sie oder ihn passen.

### Level 1 - Rechenlogik

* Treffen Sie Annahmen für Startwert und Höhe und verzichten Sie auf Kommandozeilenparameter.
* Geben Sie nur die Zwischenergebnisse aus. Z.B.:

```txt
9
18
54
216
1080
540
180
45
9
```

### Level 2 - Kommandozeilenparameter

* Fügen Sie die Möglichkeit hinzu, die Parameter über Kommandozeile zu übergeben.
* Denken Sie an die Prüfung der Parameter mit entsprechenden Fehlermeldungen.

### Level 3 - Bessere Ausgabe

* Verbessern Sie die Ausgabe, damit das Ergebnis so aussieht, wie am Beginn dieser Beschreibung gefordert.
* Geben Sie wenn möglich den Ausgangswert rechtsbündig aus.

### Level 4 - Unit Testing

* Strukturieren Sie Ihren Code so, dass Sie ihn gut testen können.
* Schreiben Sie mindestens einen sinnvollen Unit Test ([Kurzanleitung](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html))
