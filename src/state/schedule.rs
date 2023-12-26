//! Parsing the XML into something usable in Rust.
//!
//! Note that anytime `Id` is mentioned, actually the `guid` attribute is meant, **not** the `id` one.

use std::collections::{BTreeMap, HashMap};

use eyre::Result;
use time::Duration;
use uuid::Uuid;

use crate::DateTime;

#[derive(Clone, Debug)]
pub struct Schedule {
    events: HashMap<EventId, Event>,
    persons: HashMap<PersonId, Person>,

    time_map: BTreeMap<DateTime, Vec<EventId>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventId(Uuid);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Event {
    pub id: EventId,

    pub start: DateTime,
    pub duration: Duration,

    pub title: String,
    pub subtitle: String,
    pub r#abstract: String,
    pub description: String,

    pub room: String,
    pub track: String,
    pub r#type: String,

    pub language: String,
    pub url: String,
    pub feedback_url: String,
    pub links: BTreeMap<String, String>,

    pub persons: Vec<PersonId>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PersonId(Uuid);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Person {
    pub id: PersonId,
    pub name: String,
}

impl Schedule {
    pub fn new() -> Result<Self> {
        Ok(Self::placeholder())
    }

    /// Returns a placeholder schedule with only one event.
    pub fn placeholder() -> Self {
        let person_id = PersonId(Uuid::nil());
        let person = Person {
            id: person_id,
            name: "Semi-non-professional yak shaver".to_string(),
        };

        let another_person_id = PersonId(Uuid::max());
        let another_person = Person {
            id: another_person_id,
            name: "Completely inaccurate fractal".to_string(),
        };

        let event_id = EventId(Uuid::nil());
        let event_start = DateTime::now_utc();
        let event = Event {
            id: event_id,
            start: event_start,
            duration: Duration::new(3 * 30 * 60, 0),
            title: "Shaving yaks".to_string(),
            subtitle: "Important. I think.".to_string(),
            r#abstract: "We shave yaks, which is somewhat common in software development."
                .to_string(),
            description: LOREM_IPSUM.to_string(),
            room: "Somewhere I belong".to_string(),
            track: "Science".to_string(),
            r#type: "lecture".to_string(),
            language: "en".to_string(),
            url: "https://example.com".to_string(),
            feedback_url: "https://example.com".to_string(),
            links: BTreeMap::new(),
            persons: vec![person_id, another_person_id],
        };

        Self {
            events: [(event_id, event)].into_iter().collect(),
            persons: [(person_id, person), (another_person_id, another_person)]
                .into_iter()
                .collect(),
            time_map: [(event_start, vec![event_id])].into_iter().collect(),
        }
    }

    /// Returns the first event in this schedule, or `None` if the schedule contains no events.
    pub fn first(&self) -> Option<&Event> {
        let id = self
            .time_map
            .first_key_value()?
            .1
            .first()
            .expect("time map must be consistent with events");

        Some(self.resolve_event(id))
    }

    /// Return the corresponding [`Event`] for the given [`EventId`].
    ///
    /// # Panics
    ///
    /// Panics if the given [`EventId`] is not from this schedule.
    pub fn resolve_event(&self, id: &EventId) -> &Event {
        self.events
            .get(id)
            .expect("EventId to be from the current schedule")
    }

    /// Return the corresponding [`Person`] for the given [`PersonId`].
    ///
    /// # Panics
    ///
    /// Panics if the given [`PersonId`] is not from this schedule.
    pub fn resolve_person(&self, id: &PersonId) -> &Person {
        self.persons
            .get(id)
            .expect("PersonId to be from the current schedule")
    }
}

impl Event {
    pub fn end(&self) -> DateTime {
        self.start + self.duration
    }
}

const LOREM_IPSUM: &'static str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magnam aliquam quaerat voluptatem. Ut enim aeque doleamus animo, cum corpore dolemus, ﬁeri tamen permagna accessio potest, si aliquod aeternum et inﬁnitum impendere malum nobis opinemur. Quod idem licet transferre in voluptatem, ut postea variari voluptas distinguique possit, augeri ampliﬁcarique non possit. At etiam Athenis, ut e patre audiebam facete et urbane Stoicos irridente, statua est in quo a nobis philosophia defensa et collaudata est, cum id, quod maxime placeat, facere possimus, omnis voluptas assumenda est, omnis dolor repellendus. Temporibus autem quibusdam et aut oﬃciis debitis aut rerum necessitatibus saepe eveniet, ut et voluptates repudiandae sint et molestiae non recusandae. Itaque earum rerum defuturum, quas natura non depravata desiderat. Et quem ad me accedis, saluto: 'chaere,' inquam, 'Tite!' lictores, turma omnis chorusque: 'chaere, Tite!' hinc hostis mi Albucius, hinc inimicus. Sed iure Mucius. Ego autem mirari satis non queo unde hoc sit tam insolens domesticarum rerum fastidium. Non est omnino hic docendi locus; sed ita prorsus existimo, neque eum Torquatum, qui hoc primus cognomen invenerit, aut torquem illum hosti detraxisse, ut aliquam ex eo est consecutus? – Laudem et caritatem, quae sunt vitae sine metu degendae praesidia ﬁrmissima. – Filium morte multavit. – Si sine causa, nollem me ab eo delectari, quod ista Platonis, Aristoteli, Theophrasti orationis ornamenta neglexerit. Nam illud quidem physici, credere aliquid esse minimum, quod profecto numquam putavisset, si a Polyaeno, familiari suo, geometrica discere maluisset quam illum etiam ipsum dedocere. Sol Democrito magnus videtur, quippe homini erudito in geometriaque perfecto, huic pedalis fortasse; tantum enim esse omnino in nostris poetis aut inertissimae segnitiae est aut fastidii delicatissimi. Mihi quidem videtur, inermis ac nudus est. Tollit deﬁnitiones, nihil de dividendo ac partiendo docet, non quo ignorare vos arbitrer, sed ut ratione et via procedat oratio. Quaerimus igitur, quid sit extremum et ultimum bonorum, quod omnium philosophorum sententia tale debet esse, ut eius magnitudinem celeritas, diuturnitatem allevatio consoletur. Ad ea cum accedit, ut neque divinum numen horreat nec praeteritas voluptates eﬄuere patiatur earumque assidua recordatione laetetur, quid est, quod huc possit, quod melius sit, migrare de vita. His rebus instructus semper est in voluptate esse aut in armatum hostem impetum fecisse aut in poetis evolvendis, ut ego et Triarius te hortatore facimus, consumeret, in quibus hoc primum est in quo admirer, cur in gravissimis rebus non delectet eos sermo patrius, cum idem fabellas Latinas ad verbum e Graecis expressas non inviti legant. Quis enim tam inimicus paene nomini Romano est, qui Ennii Medeam aut Antiopam Pacuvii spernat aut reiciat, quod se isdem Euripidis fabulis delectari dicat, Latinas litteras oderit? Synephebos ego, inquit, potius Caecilii aut Andriam Terentii quam utramque Menandri legam? A quibus tantum dissentio, ut, cum Sophocles vel optime scripserit Electram, tamen male conversam Atilii mihi legendam putem, de quo Lucilius: 'ferreum scriptorem', verum, opinor, scriptorem tamen, ut legendus sit. Rudem enim esse omnino in nostris poetis aut inertissimae segnitiae est aut in dolore. Omnis autem privatione doloris putat Epicurus.";
