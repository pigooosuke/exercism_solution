use std::collections::HashMap;

const DNA: [char; 4] = ['A', 'C', 'G', 'T'];

fn is_nucleotide(nucleotide: &char) -> bool {
    match nucleotide {
        'A' | 'C' | 'G' | 'T' => true,
        _ => false,
    }
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !is_nucleotide(&nucleotide) {
        return Err(nucleotide);
    }
    for nucleotide in dna.chars() {
        match nucleotide {
            'A' | 'C' | 'G' | 'T' => (),
            _ => return Err(nucleotide),
        }
    }
    Ok(dna.chars().filter(|&x| x == nucleotide).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut dna_hashmap: HashMap<char, usize> = HashMap::new();
    for nucleotide in dna.chars() {
        match nucleotide {
            'A' | 'C' | 'G' | 'T' => (),
            _ => return Err(nucleotide),
        }
    }
    for nucleotide in DNA {
        dna_hashmap.insert(nucleotide, count(nucleotide, dna).unwrap_or(0));
    }
    Ok(dna_hashmap)
}
