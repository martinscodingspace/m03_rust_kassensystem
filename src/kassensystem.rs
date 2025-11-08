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


/// # Beispiele
/// ```
///  let mut artikel_vec = Vec::new();
/// ```

/// Der Kern für die obejektorierte Herangehensweise
pub struct Artikel {
    pub name: String,
    pub preis: f64,
}

/// Konstruktor-Aufruf mit neuem Struktur-Typ (Artikel) als Rückgabe-Wert
impl Artikel {
    pub fn new(name: &str, preis: f64) -> Artikel {
        Artikel {
            name: name.to_string(),
            preis,
        }
    }
}

/// Definition für das Verhalten des Datentyps in Form eines Traits [Price]
/// (Ähnlich einer abstrakten Funktion, welche die anschließende Implementierung erfordert)
/// Methode, die den Preis eines Artikels zurückgibt
pub trait Price {
    fn price(&self) -> f64; // Methode, die den Preis eines Artikels zurückgibt
}

/// Anwenden des Verhaltens (Trait) auf die entsprechende Struktur Artikel
/// Gibt den Preis des Artikels zurück
impl Price for Artikel {
    fn price(&self) -> f64 {
        self.preis
    }
}

// Anwendung des Rabattes:
// der Ursprungs-Vektor mit der definierten Closure als Eingangs-Parameter
// den Ausgangs-Vektor als Rückgabetyp
// Fn als einer der drei möglichen Closure-Traits mit unveränderbar ausgeliehenem Wert
pub fn apply_discount<F>(artikel: &Vec<Artikel>, rab_cl: F) -> Vec<Artikel>
where
    F: Fn(&Artikel) -> f64, // Einschränkung für die Anwendung
{
    // Gleichzeitig schon der Rückgabewert in Form eines abgewandelten Vektors
    // Eingangs-Liste mit iter() iterierbar gemacht am Ende mit collect() für Sammeln Zieltyp
    // Dazwischen mit map() die als Parameter erhaltene Closure
    artikel.iter().map(|a| {
        // einzelner Rabatt-Preis angwewandt auf die Closure im interierten Element
        let rabatt_preis = rab_cl(a);
        // Element erhält eine Kopie des Namens sowie den rabattierten Preis
        Artikel {
            name: a.name.clone(),
            preis: rabatt_preis,
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Hilfsfunktion, um eine Liste von Artikeln zu erstellen
    fn tt_create_articles() -> Vec<Artikel> {
        vec![
            Artikel::new("Radio", 159.99),
            Artikel::new("Fernseher", 699.99),
            Artikel::new("Beamer", 899.99),
        ]
    }

    // Test: Rabatt wird korrekt angewendet
    #[test]
    fn tt_apply_discount() {
        let artikel_liste = tt_create_articles();
        let rabattpreise = apply_discount(&artikel_liste, |a| a.price() * 0.9);

        // Überprüfen, dass der Preis korrekt angewendet wurde (10% Rabatt)
        assert_eq!(rabattpreise[0].preis, 159.99 * 0.9);
        assert_eq!(rabattpreise[1].preis, 699.99 * 0.9);
        assert_eq!(rabattpreise[2].preis, 899.99 * 0.9);
    }

    // Test: Gesamtpreis aller rabattierten Artikel
    #[test]
    fn tt_total_price() {
        let artikel_liste = tt_create_articles();
        let rabattpreise = apply_discount(&artikel_liste, |a| a.price() * 0.9);

        let gesamt: f64 = rabattpreise.iter().map(|a| a.preis).sum();
        assert!((gesamt - (159.99 * 0.9 + 699.99 * 0.9 + 899.99 * 0.9)).abs() < 1e-2);
    }

    // Test: Durchschnittspreis der rabattierten Artikel
    #[test]
    fn tt_average_price() {
        let artikel_liste = tt_create_articles();
        let rabattpreise = apply_discount(&artikel_liste, |a| a.price() * 0.9);

        let gesamt: f64 = rabattpreise.iter().map(|a| a.preis).sum();
        let durchschnitt = gesamt / rabattpreise.len() as f64;

        assert!((durchschnitt - (159.99 * 0.9 + 699.99 * 0.9 + 899.99 * 0.9) / 3.0).abs() < 1e-2);
    }

    // Test: Filtern von Artikeln über einem bestimmten Preis
    #[test]
    fn tt_filter_expensive_articles() {
        let artikel_liste = tt_create_articles();
        let rabattpreise = apply_discount(&artikel_liste, |a| a.price() * 0.9);

        let teuer = rabattpreise.iter().filter(|a| a.preis > 500.0).count();
        assert_eq!(teuer, 2); // Laptop und Smartphone sind über 500 Euro
    }

    // Test: Umgang mit leeren Listen
    #[test]
    fn tt_empty_list() {
        let empty_list: Vec<Artikel> = Vec::new();
        let rabattpreise = apply_discount(&empty_list, |a| a.price() * 0.9);

        // Keine Artikel, daher sollte die Summe und der Durchschnitt 0 sein
        assert_eq!(rabattpreise.len(), 0);
        let gesamt: f64 = rabattpreise.iter().map(|a| a.preis).sum();
        assert_eq!(gesamt, 0.0);
    }

    // Test: Sonderfall, alle Artikel unter dem Filterpreis
    #[test]
    fn tt_filter_no_expensive_articles() {
        let artikel_liste = vec![
            Artikel::new("Kopfhörer", 50.0),
            Artikel::new("Maus", 30.0),
        ];
        let rabattpreise = apply_discount(&artikel_liste, |a| a.price() * 0.9);

        // "teuer" für den Test wird hier auf 300 Euro gesetzt
        let teuer = rabattpreise.iter().filter(|a| a.preis > 300.0).count();
        assert_eq!(teuer, 0); // Keine Artikel über 300 Euro
    }
}