use std::{collections::BTreeMap, convert::Infallible, fmt, iter, str::FromStr};

pub trait LibraryItem {
    fn insert<'a, I: Iterator<Item = &'a str>>(&mut self, i: I) -> Result<(), Option<&'a str>>
    where
        Self: Sized;
    fn insert_dyn<'a>(
        &mut self,
        i: Box<dyn Iterator<Item = &'a str>>,
    ) -> Result<(), Option<&'a str>>;
    fn name(&self) -> &str;
    fn count(&self) -> usize;
    fn children<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn LibraryItem> + 'a>;
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Study {
    count: usize,
    scriptures: Scriptures,
    unknown: Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Scriptures {
    count: usize,
    bom: BoM,
    dnc: DnC,
    ot: Ot,
    nt: Nt,
    pgp: PGP,
    proc: Proclamations,
    unknown: Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BoM {
    count: usize,
    introduction: Chapter,
    ne1: Chapters,
    ne2: Chapters,
    jacob: Chapters,
    enos: Chapters,
    jarom: Chapters,
    omni: Chapters,
    wom: Chapters,
    mosiah: Chapters,
    alma: Chapters,
    helaman: Chapters,
    ne3: Chapters,
    ne4: Chapters,
    mormon: Chapters,
    ether: Chapters,
    moroni: Chapters,
    unknown: Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DnC {
    count: usize,
    title: Chapter,
    introduction: Chapter,
    chronology: Chapter,
    sections: Chapters,
    declarations: Chapters,
    unknown: Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Ot {
    count: usize,
    genesis: Chapters,
    exodus: Chapters,
    leviticus: Chapters,
    numbers: Chapters,
    deut: Chapters,
    joshua: Chapters,
    judges: Chapters,
    ruth: Chapters,
    sam1: Chapters,
    sam2: Chapters,
    kings1: Chapters,
    kings2: Chapters,
    chron1: Chapters,
    chron2: Chapters,
    ezra: Chapters,
    neh: Chapters,
    esther: Chapters,
    job: Chapters,
    psalms: Chapters,
    proverbs: Chapters,
    eccles: Chapters,
    solomon_song: Chapters,
    isaiah: Chapters,
    jeremiah: Chapters,
    lament: Chapters,
    ezekiel: Chapters,
    daniel: Chapters,
    hosea: Chapters,
    joel: Chapters,
    amos: Chapters,
    obadiah: Chapter,
    jonah: Chapters,
    micah: Chapters,
    nahum: Chapters,
    habak: Chapters,
    zeph: Chapters,
    haggai: Chapters,
    zech: Chapters,
    malachi: Chapters,
    unknown: Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Nt {
    count: usize,
    title: Chapter,
    matt: Chapters,
    mark: Chapters,
    luke: Chapters,
    john: Chapters,
    acts: Chapters,
    romans: Chapters,
    cor1: Chapters,
    cor2: Chapters,
    galatians: Chapters,
    ephesians: Chapters,
    philippians: Chapters,
    colossians: Chapters,
    thess1: Chapters,
    thess2: Chapters,
    tim1: Chapters,
    tim2: Chapters,
    titus: Chapters,
    philemon: Chapters,
    heb: Chapters,
    james: Chapters,
    peter1: Chapters,
    peter2: Chapters,
    john1: Chapters,
    john2: Chapters,
    john3: Chapters,
    jude: Chapters,
    revelation: Chapters,
    unknown: Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PGP {
    count: usize,
    moses: Chapters,
    abraham: Chapters,
    js_matt: Chapters,
    js_hist: Chapters,
    aof: Chapter,
    unknown: Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Proclamations {}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Verse(pub String, pub usize);

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Chapter {
    pub name: String,
    pub count: usize,
    pub verses: BTreeMap<usize, Verse>,
    pub unknown: Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Chapters {
    pub name: String,
    pub count: usize,
    pub chapters: BTreeMap<usize, Chapter>,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Highlight;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unknown(pub String, pub BTreeMap<String, Unknown>, usize);

impl LibraryItem for Study {
    fn insert<'a, I: Iterator<Item = &'a str>>(&mut self, mut i: I) -> Result<(), Option<&'a str>> {
        let next = i.next().ok_or(None)?;
        self.count += 1;
        match next {
            "scriptures" => self.scriptures.insert(i),
            _ => self.unknown.insert(i),
        }
    }

    fn insert_dyn<'a>(
        &mut self,
        i: Box<dyn Iterator<Item = &'a str>>,
    ) -> Result<(), Option<&'a str>> {
        self.insert(i)
    }

    fn name(&self) -> &str {
        "Study"
    }

    fn count(&self) -> usize {
        self.count
    }

    fn children<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn LibraryItem> + 'a> {
        Box::new([&self.scriptures as _, &self.unknown as _].into_iter())
    }
}

impl LibraryItem for Scriptures {
    fn insert<'a, I: Iterator<Item = &'a str>>(&mut self, mut i: I) -> Result<(), Option<&'a str>> {
        let next = i.next().ok_or(None)?;
        self.count += 1;
        match next {
            "bofm" => self.bom.insert(i),
            "ot" => self.ot.insert(i),
            "nt" => self.nt.insert(i),
            "dc-testament" => self.dnc.insert(i),
            "pgp" => self.pgp.insert(i),
            _ => self.unknown.insert(iter::once(next).chain(i)),
        }
    }

    fn insert_dyn<'a>(
        &mut self,
        i: Box<dyn Iterator<Item = &'a str>>,
    ) -> Result<(), Option<&'a str>> {
        self.insert(i)
    }

    fn name(&self) -> &str {
        "Scriptures"
    }

    fn count(&self) -> usize {
        self.count
    }

    fn children<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn LibraryItem> + 'a> {
        Box::new(
            [
                &self.bom as _,
                &self.dnc as _,
                &self.ot as _,
                &self.nt as _,
                &self.pgp as _,
                //&self.proc as _,
                &self.unknown as _,
            ]
            .into_iter(),
        )
    }
}

impl LibraryItem for BoM {
    fn insert<'a, I: Iterator<Item = &'a str>>(&mut self, mut i: I) -> Result<(), Option<&'a str>> {
        let next = i.next().ok_or(None)?;
        self.count += 1;
        match next {
            "introduction" => self.introduction.insert(i),
            "1-ne" => self.ne1.insert(i),
            "2-ne" => self.ne2.insert(i),
            "jacob" => self.jacob.insert(i),
            "enos" => self.enos.insert(i),
            "jarom" => self.jarom.insert(i),
            "omni" => self.omni.insert(i),
            "w-of-m" => self.wom.insert(i),
            "mosiah" => self.mosiah.insert(i),
            "alma" => self.alma.insert(i),
            "hel" => self.helaman.insert(i),
            "3-ne" => self.ne3.insert(i),
            "4-ne" => self.ne4.insert(i),
            "morm" => self.mormon.insert(i),
            "ether" => self.ether.insert(i),
            "moro" => self.moroni.insert(i),
            _ => self.unknown.insert(iter::once(next).chain(i)),
        }
    }

    fn insert_dyn<'a>(
        &mut self,
        i: Box<dyn Iterator<Item = &'a str>>,
    ) -> Result<(), Option<&'a str>> {
        self.insert(i)
    }

    fn name(&self) -> &str {
        "Book of Mormon"
    }

    fn count(&self) -> usize {
        self.count
    }

    fn children<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn LibraryItem> + 'a> {
        Box::new(
            [
                &self.introduction as _,
                &self.ne1 as _,
                &self.ne2 as _,
                &self.jacob as _,
                &self.enos as _,
                &self.jarom as _,
                &self.omni as _,
                &self.wom as _,
                &self.mosiah as _,
                &self.alma as _,
                &self.helaman as _,
                &self.ne3 as _,
                &self.ne4 as _,
                &self.mormon as _,
                &self.ether as _,
                &self.moroni as _,
                &self.unknown as _,
            ]
            .into_iter(),
        )
    }
}

impl LibraryItem for Ot {
    fn insert<'a, I: Iterator<Item = &'a str>>(&mut self, mut i: I) -> Result<(), Option<&'a str>> {
        let next = i.next().ok_or(None)?;
        self.count += 1;
        match next {
            "gen" => self.genesis.insert(i),
            "ex" => self.exodus.insert(i),
            "lev" => self.leviticus.insert(i),
            "num" => self.numbers.insert(i),
            "deut" => self.deut.insert(i),
            "josh" => self.joshua.insert(i),
            "judg" => self.judges.insert(i),
            "ruth" => self.ruth.insert(i),
            "1-sam" => self.sam1.insert(i),
            "2-sam" => self.sam2.insert(i),
            "1-kgs" => self.kings1.insert(i),
            "2-kgs" => self.kings2.insert(i),
            "1-chr" => self.chron1.insert(i),
            "2-chr" => self.chron2.insert(i),
            "ezra" => self.ezra.insert(i),
            "neh" => self.neh.insert(i),
            "esth" => self.esther.insert(i),
            "job" => self.job.insert(i),
            "ps" => self.psalms.insert(i),
            "prov" => self.proverbs.insert(i),
            "eccl" => self.eccles.insert(i),
            "song" => self.solomon_song.insert(i),
            "isa" => self.isaiah.insert(i),
            "jer" => self.jeremiah.insert(i),
            "lam" => self.lament.insert(i),
            "ezek" => self.ezekiel.insert(i),
            "dan" => self.daniel.insert(i),
            "hosea" => self.hosea.insert(i),
            "joel" => self.joel.insert(i),
            "amos" => self.amos.insert(i),
            "obad" => self.obadiah.insert(i),
            "jonah" => self.jonah.insert(i),
            "micah" => self.micah.insert(i),
            "nahum" => self.nahum.insert(i),
            "hab" => self.habak.insert(i),
            "zeph" => self.zeph.insert(i),
            "hag" => self.haggai.insert(i),
            "zech" => self.zech.insert(i),
            "mal" => self.malachi.insert(i),
            _ => self.unknown.insert(iter::once(next).chain(i)),
        }
    }

    fn insert_dyn<'a>(
        &mut self,
        i: Box<dyn Iterator<Item = &'a str>>,
    ) -> Result<(), Option<&'a str>> {
        self.insert(i)
    }

    fn name(&self) -> &str {
        "Old Testament"
    }

    fn count(&self) -> usize {
        self.count
    }

    fn children<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn LibraryItem> + 'a> {
        Box::new(
            [
                &self.genesis as _,
                &self.exodus as _,
                &self.leviticus as _,
                &self.numbers as _,
                &self.deut as _,
                &self.joshua as _,
                &self.judges as _,
                &self.ruth as _,
                &self.sam1 as _,
                &self.sam2 as _,
                &self.kings1 as _,
                &self.kings2 as _,
                &self.chron1 as _,
                &self.chron2 as _,
                &self.ezra as _,
                &self.neh as _,
                &self.esther as _,
                &self.job as _,
                &self.psalms as _,
                &self.proverbs as _,
                &self.eccles as _,
                &self.solomon_song as _,
                &self.isaiah as _,
                &self.jeremiah as _,
                &self.lament as _,
                &self.ezekiel as _,
                &self.daniel as _,
                &self.hosea as _,
                &self.joel as _,
                &self.amos as _,
                &self.obadiah as _,
                &self.jonah as _,
                &self.micah as _,
                &self.nahum as _,
                &self.habak as _,
                &self.zeph as _,
                &self.haggai as _,
                &self.zech as _,
                &self.malachi as _,
                &self.unknown as _,
            ]
            .into_iter(),
        )
    }
}

impl LibraryItem for Nt {
    fn insert<'a, I: Iterator<Item = &'a str>>(&mut self, mut i: I) -> Result<(), Option<&'a str>> {
        let next = i.next().ok_or(None)?;
        self.count += 1;
        match next {
            "title-page" => self.title.insert(i),
            "matt" => self.matt.insert(i),
            "mark" => self.mark.insert(i),
            "luke" => self.luke.insert(i),
            "john" => self.john.insert(i),
            "acts" => self.acts.insert(i),
            "rom" => self.romans.insert(i),
            "1-cor" => self.cor1.insert(i),
            "2-cor" => self.cor2.insert(i),
            "gal" => self.galatians.insert(i),
            "eph" => self.ephesians.insert(i),
            "philip" => self.philippians.insert(i),
            "col" => self.colossians.insert(i),
            "1-thes" => self.thess1.insert(i),
            "2-thes" => self.thess2.insert(i),
            "1-tim" => self.tim1.insert(i),
            "2-tim" => self.tim2.insert(i),
            "titus" => self.titus.insert(i),
            "philem" => self.philemon.insert(i),
            "heb" => self.heb.insert(i),
            "james" => self.james.insert(i),
            "1-pet" => self.peter1.insert(i),
            "2-pet" => self.peter2.insert(i),
            "1-jn" => self.john1.insert(i),
            "2-jn" => self.john2.insert(i),
            "3-jn" => self.john3.insert(i),
            "jude" => self.jude.insert(i),
            "rev" => self.revelation.insert(i),
            _ => self.unknown.insert(iter::once(next).chain(i)),
        }
    }

    fn insert_dyn<'a>(
        &mut self,
        i: Box<dyn Iterator<Item = &'a str>>,
    ) -> Result<(), Option<&'a str>> {
        self.insert(i)
    }

    fn name(&self) -> &str {
        "New Testament"
    }

    fn count(&self) -> usize {
        self.count
    }

    fn children<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn LibraryItem> + 'a> {
        Box::new(
            [
                &self.title as _,
                &self.matt as _,
                &self.mark as _,
                &self.luke as _,
                &self.john as _,
                &self.acts as _,
                &self.romans as _,
                &self.cor1 as _,
                &self.cor2 as _,
                &self.galatians as _,
                &self.ephesians as _,
                &self.philippians as _,
                &self.colossians as _,
                &self.thess1 as _,
                &self.thess2 as _,
                &self.tim1 as _,
                &self.tim2 as _,
                &self.titus as _,
                &self.philemon as _,
                &self.heb as _,
                &self.james as _,
                &self.peter1 as _,
                &self.peter2 as _,
                &self.john1 as _,
                &self.john2 as _,
                &self.john3 as _,
                &self.jude as _,
                &self.revelation as _,
                &self.unknown as _,
            ]
            .into_iter(),
        )
    }
}

impl LibraryItem for DnC {
    fn insert<'a, I: Iterator<Item = &'a str>>(&mut self, mut i: I) -> Result<(), Option<&'a str>> {
        let next = i.next().ok_or(None)?;
        self.count += 1;
        match next {
            "title-page" => self.title.insert(i),
            "chron-order" => self.chronology.insert(i),
            "introduction" => self.introduction.insert(i),
            "dc" => self.sections.insert(i),
            "od" => self.declarations.insert(i),
            _ => self.unknown.insert(iter::once(next).chain(i)),
        }
    }

    fn insert_dyn<'a>(
        &mut self,
        i: Box<dyn Iterator<Item = &'a str>>,
    ) -> Result<(), Option<&'a str>> {
        self.insert(i)
    }

    fn name(&self) -> &str {
        "Doctrine and Covenants"
    }

    fn count(&self) -> usize {
        self.count
    }

    fn children<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn LibraryItem> + 'a> {
        Box::new(
            [
                &self.title as _,
                &self.introduction as _,
                &self.chronology as _,
                &self.sections as _,
                &self.declarations as _,
                &self.unknown as _,
            ]
            .into_iter(),
        )
    }
}

impl LibraryItem for PGP {
    fn insert<'a, I: Iterator<Item = &'a str>>(&mut self, mut i: I) -> Result<(), Option<&'a str>> {
        let next = i.next().ok_or(None)?;
        self.count += 1;
        match next {
            "moses" => self.moses.insert(i),
            "abr" => self.abraham.insert(i),
            "js-m" => self.js_matt.insert(i),
            "js-h" => self.js_hist.insert(i),
            "a-of-f" => self.aof.insert(i),
            _ => self.unknown.insert(iter::once(next).chain(i)),
        }
    }

    fn insert_dyn<'a>(
        &mut self,
        i: Box<dyn Iterator<Item = &'a str>>,
    ) -> Result<(), Option<&'a str>> {
        self.insert(i)
    }

    fn name(&self) -> &str {
        "Pearl of Great Price"
    }

    fn count(&self) -> usize {
        self.count
    }

    fn children<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn LibraryItem> + 'a> {
        Box::new(
            [
                &self.moses as _,
                &self.abraham as _,
                &self.js_matt as _,
                &self.js_hist as _,
                &self.aof as _,
                &self.unknown as _,
            ]
            .into_iter(),
        )
    }
}

impl LibraryItem for Chapters {
    fn insert<'a, I: Iterator<Item = &'a str>>(&mut self, mut i: I) -> Result<(), Option<&'a str>> {
        let next = i.next().ok_or(None)?;
        let n = next.parse::<usize>().map_err(|_| next)?;
        self.count += 1;
        self.chapters.entry(n).or_insert(Chapter::new(n)).insert(i)
    }

    fn insert_dyn<'a>(
        &mut self,
        i: Box<dyn Iterator<Item = &'a str>>,
    ) -> Result<(), Option<&'a str>> {
        self.insert(i)
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn count(&self) -> usize {
        self.count
    }

    fn children<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn LibraryItem> + 'a> {
        Box::new(self.chapters.values().map(|v| v as &'a dyn LibraryItem))
    }
}

impl Chapter {
    fn new(n: usize) -> Self {
        Chapter {
            name: format!("Chapter {n}"),
            count: 0,
            verses: BTreeMap::new(),
            unknown: Unknown::default(),
        }
    }
}

impl LibraryItem for Chapter {
    fn insert<'a, I: Iterator<Item = &'a str>>(&mut self, mut i: I) -> Result<(), Option<&'a str>> {
        self.count += 1;
        let next = i.next().ok_or(None)?;
        let n = next
            .split_once("id=p")
            .and_then(|(_, n)| n.parse::<usize>().ok());
        match n {
            Some(n) => self.verses.entry(n).or_insert(Verse::new(n)).insert(i)?,
            None => self.unknown.insert(iter::once(next).chain(i))?,
        }
        Ok(())
    }

    fn insert_dyn<'a>(
        &mut self,
        i: Box<dyn Iterator<Item = &'a str>>,
    ) -> Result<(), Option<&'a str>> {
        self.insert(i)
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn count(&self) -> usize {
        self.count
    }

    fn children<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn LibraryItem> + 'a> {
        Box::new(self.verses.values().map(|v| v as &'a dyn LibraryItem))
    }
}

impl Verse {
    fn new(n: usize) -> Verse {
        Verse(format!("Verse {n}"), 0)
    }
}

impl LibraryItem for Verse {
    fn insert<'a, I: Iterator<Item = &'a str>>(&mut self, _i: I) -> Result<(), Option<&'a str>> {
        self.1 += 1;
        Ok(())
    }

    fn insert_dyn<'a>(
        &mut self,
        i: Box<dyn Iterator<Item = &'a str>>,
    ) -> Result<(), Option<&'a str>> {
        self.insert(i)
    }

    fn name(&self) -> &str {
        &self.0
    }

    fn count(&self) -> usize {
        self.1
    }

    fn children<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn LibraryItem> + 'a> {
        Box::new(iter::empty())
    }
}

impl Unknown {
    fn new(name: String) -> Self {
        Unknown(name, BTreeMap::new(), 0)
    }
}

impl Default for Unknown {
    fn default() -> Self {
        Unknown::new("Unknown".into())
    }
}

impl LibraryItem for Unknown {
    fn insert<'a, I: Iterator<Item = &'a str>>(&mut self, mut i: I) -> Result<(), Option<&'a str>> {
        self.2 += 1;
        match i.next() {
            Some(n) => (self.1.entry(n.into()))
                .or_insert_with_key(|k| Unknown::new(k.into()))
                .insert(i),
            None => Ok(()),
        }
    }

    fn insert_dyn<'a>(
        &mut self,
        i: Box<dyn Iterator<Item = &'a str>>,
    ) -> Result<(), Option<&'a str>> {
        self.insert(i)
    }

    fn name(&self) -> &str {
        &self.0
    }

    fn count(&self) -> usize {
        self.2
    }

    fn children<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn LibraryItem> + 'a> {
        Box::new(self.1.values().map(|v| v as &'a dyn LibraryItem))
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Segment {
    Chapter(u32),
    Name(String),
}

impl FromStr for Segment {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u32>() {
            Ok(n) => Ok(Segment::Chapter(n)),
            Err(_) => Ok(Segment::Name(String::from(s))),
        }
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Segment::Chapter(n) => write!(f, "{n}"),
            Segment::Name(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct UrlAnalytics {
    total_count: usize,
    children: BTreeMap<Segment, UrlAnalytics>,
}

impl UrlAnalytics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_url(&mut self, path: &str) {
        let (_, path) = path.split_once("/study/").unwrap_or(("", path));
        self.add_segments(path.split('/').flat_map(|s| s.split("?")));
    }

    pub fn add_segments<'a, I: Iterator<Item = &'a str>>(&mut self, mut i: I) {
        let Some(segment) = i.next() else {
            return;
        };

        self.total_count += 1;
        self.children
            .entry(segment.parse().unwrap())
            .or_default()
            .add_segments(i);
    }

    fn format_in(&self, depth: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (segment, data) in &self.children {
            if data.total_count > 0 {
                for _ in 0..depth {
                    write!(f, "  ")?;
                }
                writeln!(f, "{segment} ({})", data.total_count)?;
                data.format_in(depth + 1, f)?;
            }
        }

        Ok(())
    }
}

impl fmt::Display for UrlAnalytics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Total: {}", self.total_count)?;
        self.format_in(1, f)
    }
}
