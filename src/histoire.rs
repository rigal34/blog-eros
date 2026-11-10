// Le contenu du blog : la fiche d'Eros et les chapitres de son histoire.
// Pour modifier un texte, change-le ici. Pour ajouter un chapitre,
// ajoute un bloc Chapitre { ... } à la fin de la liste CHAPITRES.

#[derive(PartialEq)]
pub struct Chapitre {
    pub numero: u8,
    pub etape: &'static str,
    pub titre: &'static str,
    pub image: &'static str,
    pub legende: &'static str,
    pub paragraphes: &'static [&'static str],
    pub encadre: Option<Encadre>,
}

#[derive(PartialEq)]
pub struct Encadre {
    pub etiquette: &'static str,
    pub texte: &'static str,
}

pub const FICHE: &[(&str, &str)] = &[
    ("Nom", "Eros"),
    ("Race", "American Staffordshire terrier"),
    ("Catégorie", "2, chien de garde et de défense"),
    ("Provenance", "Refuge"),
    ("Évaluation comportementale", "Niveau 1"),
    ("Statut", "Adopté par Bruno"),
];

pub const CHAPITRES: &[Chapitre] = &[
    Chapitre {
        numero: 1,
        etape: "Au refuge",
        titre: "La rencontre",
        image: "images/chapitre-1.jpg",
        legende: "Eros au refuge.",
        paragraphes: &[
            "La première fois que j'ai vu Eros, il était au fond de son box, au refuge. Il ne venait pas vers la grille comme les autres chiens. Il me regardait sans bouger, comme s'il attendait de savoir ce que j'allais faire.",
            "Les bénévoles m'ont raconté le peu qu'ils savaient de lui. Ce jour-là, j'ai compris que ce chien avait besoin de quelqu'un qui ne le lâcherait pas.",
        ],
        encadre: None,
    },
    Chapitre {
        numero: 2,
        etape: "Avant le refuge",
        titre: "Ce qu'il avait vécu",
        image: "images/chapitre-2.jpg",
        legende: "Les traces de son passé.",
        paragraphes: &[
            "Ses anciens propriétaires se servaient de lui pour des combats de chiens. Eros n'avait connu ni la douceur ni le jeu. On l'avait obligé à se battre.",
            "Quand je l'ai récupéré, il était en très mauvais état. Son corps portait les marques de ce qu'il avait subi.",
        ],
        encadre: Some(Encadre {
            etiquette: "Ce que dit la loi",
            texte: "Organiser des combats de chiens est interdit en France. Les mauvais traitements envers les animaux sont punis d'amende et de prison.",
        }),
    },
    Chapitre {
        numero: 3,
        etape: "La maladie",
        titre: "Entre la vie et la mort",
        image: "images/chapitre-3.webp",
        legende: "Eros au barbecue.",
        paragraphes: &[
            "Eros avait une infection aux poumons. Il était faible et il respirait mal. Pendant plusieurs jours, il a été entre la vie et la mort.",
            "Je ne savais pas s'il allait s'en sortir. Je suis resté près de lui. Je voulais qu'il sache qu'il n'était plus seul.",
        ],
        encadre: None,
    },
    Chapitre {
        numero: 4,
        etape: "La guérison",
        titre: "Le retour à la vie",
        image: "images/chapitre-4.jpg",
        legende: "Eros reprend des forces.",
        paragraphes: &[
            "Eros s'est battu, cette fois pour vivre. Petit à petit, il a repris des forces.",
            "La confiance est venue ensuite, lentement. Un regard, puis un pas vers moi. Ce chien qu'on avait forcé à se battre découvrait qu'une main pouvait aussi le caresser.",
        ],
        encadre: None,
    },
    Chapitre {
        numero: 5,
        etape: "Aujourd'hui",
        titre: "Eros aujourd'hui",
        image: "images/chapitre-5.jpg",
        legende: "Eros aujourd'hui.",
        paragraphes: &[
            "Aujourd'hui, Eros vit avec moi. Il est assuré, j'ai obtenu son permis de détention, et il a passé son évaluation comportementale : niveau 1, le niveau le plus bas.",
            "Dehors, il porte toujours sa laisse et sa muselière, comme la loi le demande pour les chiens de catégorie 2. Les personnes qui le connaissent le disent : Eros est un chien très gentil, qui joue volontiers avec les autres chiens.",
        ],
        encadre: None,
    },
];

pub const CHIENS_CATEGORIE_2: &str = "L'American Staffordshire terrier avec pedigree, le Rottweiler et les chiens qui lui ressemblent, et le Tosa.";

pub const OBLIGATIONS: &[(&str, &str)] = &[
    ("Identification", "Une puce électronique enregistrée au fichier national (I-CAD)."),
    ("Vaccin contre la rage", "À jour, inscrit dans le passeport européen du chien."),
    ("Attestation d'aptitude", "Une journée de formation suivie par le maître."),
    ("Évaluation comportementale", "Réalisée par un vétérinaire agréé."),
    ("Assurance", "Une responsabilité civile qui couvre les dommages causés par le chien."),
    ("Permis de détention", "Délivré gratuitement par la mairie, un permis par chien."),
    ("Laisse et muselière", "Sur la voie publique, dans les parties communes des immeubles, les lieux publics et les transports."),
];

pub const NIVEAUX: &[(u8, &str)] = &[
    (1, "Pas de risque particulier, en dehors de ceux de tout chien."),
    (2, "Risque faible, pour certaines personnes ou certaines situations."),
    (3, "Risque critique, pour certaines personnes ou certaines situations."),
    (4, "Risque élevé, pour certaines personnes."),
];

pub const NIVEAU_EROS: u8 = 1;

// Les photos récentes, montrées dans « Les nouvelles d'Eros » : (fichier, légende).
pub const GALERIE: &[(&str, &str)] = &[
    ("images/eros-assis.webp", "Eros assis."),
    ("images/barbecue.webp", "Eros surveille le barbecue."),
    ("images/corbeille.webp", "Eros dans la corbeille de sa copine Capucine."),
];
