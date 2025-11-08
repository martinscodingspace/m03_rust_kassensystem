//! Das Programm m03_rust_kassensystem.rs beinhaltet folgenden Zweck:
//!
//! Entwicklung eines einfaches Kassensystem, das eine Liste von Artikeln
//! oder Dienstleistungen verwaltet und Rabatte berechnet.
//!
//! Nach erfolgter (1.) Vorbereitung (Anlegen einer Datenstruktur)
//! werden (2.) Rabatte berechnet – mit Hilfe von Generics und Closures
//! Durch die Nutzung von (3.) Iteratoren für Auswertungen werden folgende
//! Auswertungen durchgeführt:
//! • Berechnung des Gesamtpreises aller rabattierten Artikel.
//! • Ermittlung des Durchschnittspreises.
//! • Filterung alle Artikel über einem bestimmten Preis heraus.
//! Zwecks erfolgreicher Überprüfung werden Unit-Tests geschrieben,
//! welche das Kassensystem in seiner Funktionalität überprüfen
//! Im Rahmen der Erweiterungsmöglichkeiten kann unter anderem (5.) ein kleines Menü
//! geschrieben werden, was über die Konsole Artikelpreise und Rabattwerte
//! einliest. Des weiteren ist eine Funktion möglich, die automatisch den höchsten
//! Rabatt auswählt.

/********************************************************************
***  IHK Rust Developer 2025/2026                                 ***
***  m03_rust_kassensystem                                        ***
***  Fälligkeit 08.11.2025 23:59                                  ***
***  written by Martin Hildebrand                                 ***
***  2025 ©  ALL RIGHTS RESERVED                                  ***
*********************************************************************
***  Das Repository befindet sich hier:                           ***
***  https://github.com/martinscodingspace/m03_rust_kassensystem  ***
*********************************************************************
***  Diese Software steht unter folgender LIZENZ                  ***
***  GNU General Public License 3                                 ***
***  http://www.gnu.org/licenses/gpl-3.0.de.html                  ***
********************************************************************/

mod kassensystem;
use std::io::{self, Write};
use crate::kassensystem::Price;
use crate::kassensystem::apply_discount;

fn main() {
    // Variablen: leerer Vektor für Struktur-Typ Artikel und Eingabe-Zähler
    let mut artikel_vec = Vec::new();
    let mut counter: u8 = 1;
    println!("\n*****************************************************************************************");
    println!("*****************************************************************************************");
    println!("Programm m03_rust_kassensystem");
    println!("Eine Liste von Artikeln oder Dienstleistungen wird verwaltet \
              und Rabatte werden berechnet");
    println!("*****************************************************************************************");
    loop {
        // Das Menu zum Einlesen: Abbruchbedingung mit 'X'
        print!("Bezeichnung des {}. Artikels  <Abruch mit 'X'>    :   ", counter);
        io::stdout().flush().unwrap();
        let mut bez = String::new();
        io::stdin().read_line(&mut bez).expect("Fehler beim Einlesen");
        bez = bez.trim().to_string();
        // Abbruchbedingung somit Schleifenende
        if bez == "X" || bez == "x" { break; }
        print!("Preis des Artikels                               :   ");
        io::stdout().flush().unwrap();
        let mut prc = String::new();
        io::stdin().read_line(&mut prc).expect("Fehler beim Einlesen");
        // Matchen mit Enum-Result, ob die Umwandlung von Text in Zahl erfolgreich ist
        let prc : f64 = match prc.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nFehler beim Umwandeln in eine Zahl - bitte erneute Eingabe\n");
                continue;
            },
        };
        // Im Erfolgsfall wird ein neuer Artikel generiert ...
        let artikel_neu = kassensystem::Artikel::new(&bez, prc);
        // ... und im Anschluss dem Artikel-Vektor hinzugefügt
        artikel_vec.push(artikel_neu);
        // Zähler für den nächsten Schleifendurchlauf als Orientierungshilfe für den Nutzer
        counter += 1;
        println!("*****************************************************************************************");
    }

    // Nur im Fall eines nicht leeren Vektors
    if artikel_vec.len() != 0 {
        println!("*****************************************************************************************");
        println!("*****************************************************************************************");
        println!("Vielen Dank  -  Es folgt nun die Berechnung");
        println!("*****************************************************************************************");
        // Abfrage wie hoch der Rabatt sein soll
        print!("Wie hoch in Prozent soll der Rabatt-Satz sein?   :   ");
        io::stdout().flush().unwrap();
        let mut rabatt_satz = String::new();
        io::stdin().read_line(&mut rabatt_satz).expect("Fehler beim Einlesen");
        // Default-Wert 0 gilt automatisch, wenn der Rabattsatz nicht richtig eingegeben wurde
        // Durch unwrap_or(0) anstelle eines Enum-Result ist ein Fehler hier abgefangen
        let rabatt_satz: f64 = rabatt_satz.trim().parse().unwrap_or(0.0);
        if rabatt_satz == 0.0 {
            println!("*****************************************************************************************");
            println!("Keine gültige Eingabe !!!  Der Rabattsatz ist somit automatisch 0.0 Prozent");
            println!("*****************************************************************************************");
        }
        println!("*****************************************************************************************");

        // Abfrage ab wann 'teuer" zutreffend ist
        print!("Ab wieviel Euro soll es als \"teuer\" gelten?      :   ");
        io::stdout().flush().unwrap();
        let mut too_much = String::new();
        io::stdin().read_line(&mut too_much).expect("Fehler beim Einlesen");
        // Auch hier Default-Wert gesetzt, wenn nicht umgewandelt werden konnte: ab 100 Euro alles 'teuer'
        let too_much: f64 = too_much.trim().parse().unwrap_or(100.0);
        if too_much == 100.0 {
            println!("*****************************************************************************************");
            println!("Keine gültige Eingabe !!!  'Teuer' wird somit automatisch auf 100,00 Euro definiert");
            println!("*****************************************************************************************");
        }
        println!("*****************************************************************************************");

        // Anwenden des individuell gewünschten Rabattes
        let rabattpreise = apply_discount(&artikel_vec, |a| a.price() * (1.0 - rabatt_satz/100.0));

        // Ausgabe der rabattierten Preise
        for artikel in &rabattpreise {
            println!("\nDer rabattierte Preis für '{}' beträgt   \t:\t{:.2}\tEuro", artikel.name, artikel.preis);
        }

        // Ausgabe Gesamtpreis aller rabattierten Artikel
        let gesamt: f64 = rabattpreise.iter().map(|a| a.preis).sum();
        println!("\nGesamtpreis aller rabattierten Artikel\t\t :\t{:.2}\tEuro\n", gesamt);

        // Ausgabe Durchschnittspreis der rabattierten Artikel
        let durchschnitt = gesamt / rabattpreise.len() as f64;
        println!("Durchschnittspreis der rabattierten Artikel\t :\t{:.2}\tEuro\n", durchschnitt);

        // Ausgabe Anzahl der Artikel über einem bestimmten Preis
        let teuer = rabattpreise.iter().filter(|a| a.preis > too_much).count();
        println!("Anzahl der Artikel über {} Euro            \t :\t{}\n", too_much, teuer);
        println!("*****************************************************************************************");
    }
    // Programm-ENDE
    println!("Ende des Programms m03_rust_kassensystem");
    println!("*****************************************************************************************");
    println!("*****************************************************************************************\n\n");
}