use yew::prelude::*;

use crate::histoire::{
    Chapitre, CHAPITRES, CHIENS_CATEGORIE_2, FICHE, GALERIE, NIVEAUX, NIVEAU_EROS, OBLIGATIONS,
};

// Où scintillent les étoiles sur le chrome de la plaque : (gauche en %, haut en %, délai en secondes)
const ETINCELLES: &[(f32, f32, f32)] = &[
    (19.8, 22.3, 0.3),
    (11.6, 32.8, 0.9),
    (47.6, 23.3, 1.5),
    (34.0, 64.8, 2.1),
    (13.2, 3.0, 2.7),
    (89.3, 33.0, 3.3),
    (57.4, 23.8, 3.9),
    (92.6, 84.9, 4.5),
];

#[component]
pub fn App() -> Html {
    html! {
        <>
            <Entete />
            <Couverture />
            <div class="parcours">
                <Sommaire />
                <main class="recit">
                    { for CHAPITRES.iter().map(|chapitre| html! { <ChapitreVue chapitre={chapitre} /> }) }
                </main>
            </div>
            <Comprendre />
            <Nouvelles />
            <PiedDePage />
            <crate::empreintes::Empreintes />
        </>
    }
}

#[component]
fn Entete() -> Html {
    html! {
        <header class="entete">
            <div class="entete-int">
                <a class="marque" href="#haut">{ "Le blog d'Eros" }</a>
                <nav class="entete-nav" aria-label="Navigation principale">
                    <a href="#chapitre-1">{ "L'histoire" }</a>
                    <a href="#comprendre">{ "La catégorie 2" }</a>
                </nav>
            </div>
        </header>
    }
}

#[component]
fn Couverture() -> Html {
    html! {
        <section class="couverture" id="haut">
            <div class="couverture-int">
                <div class="couverture-texte">
                    <h1 class="titre-eros">
                        // La plaque chromée en forme d'os, avec le nom d'Eros et Cupidon
                        <span class="plaque">
                            <img src="images/plaque-eros-cupidon.webp" alt="Eros" width="1200" height="403" />
                            // Le reflet qui passe sur le chrome, de temps en temps
                            <span class="plaque-reflet" aria-hidden="true"></span>
                            // Les petites étoiles qui scintillent sur le chrome : lettres, pattes et contour, jamais sur Cupidon
                            { for ETINCELLES.iter().map(|&(gauche, haut, delai)| html! {
                                <span
                                    class="etincelle"
                                    aria-hidden="true"
                                    style={format!("left:{}%;top:{}%;animation-delay:{}s;", gauche, haut, delai)}
                                ></span>
                            }) }
                        </span>
                    </h1>
                    <p class="surtitre">{ "Une histoire vraie" }</p>
                    <p class="chapeau">
                        { "Il a connu les combats, la peur et une infection qui a failli l'emporter. Voici son histoire, racontée par celui qui l'a sorti du refuge." }
                    </p>
                    <a class="bouton" href="#chapitre-1">{ "Lire son histoire" }</a>
                </div>
                <figure class="couverture-photo">
                    <img src="images/couverture.webp" alt="Portrait d'Eros" width="1200" height="1500" />
                    <FicheAccueil />
                </figure>
            </div>
        </section>
    }
}

#[component]
fn FicheAccueil() -> Html {
    html! {
        <div class="fiche">
            // Le papier du ticket de caisse, découpé en dents de scie en haut et en bas
            <div class="fiche-papier">
                <p class="fiche-titre">{ "Fiche d'accueil" }</p>
                <dl>
                    { for FICHE.iter().map(|&(cle, valeur)| html! {
                        <div class="fiche-ligne">
                            <dt>{ cle }</dt>
                            <dd>{ valeur }</dd>
                        </div>
                    }) }
                </dl>
            </div>
        </div>
    }
}

#[component]
fn Sommaire() -> Html {
    html! {
        <nav class="sommaire" aria-label="Les étapes de l'histoire">
            <p class="sommaire-titre">{ "Les étapes" }</p>
            <ol>
                { for CHAPITRES.iter().map(|chapitre| html! {
                    <li>
                        <a href={format!("#chapitre-{}", chapitre.numero)}>
                            <span class="sommaire-num">{ chapitre.numero }</span>
                            <span class="sommaire-texte">
                                <span class="sommaire-etape">{ chapitre.etape }</span>
                                <span class="sommaire-chap">{ chapitre.titre }</span>
                            </span>
                        </a>
                    </li>
                }) }
            </ol>
        </nav>
    }
}

#[derive(Properties, PartialEq)]
struct ChapitreProps {
    chapitre: &'static Chapitre,
}

#[component]
fn ChapitreVue(props: &ChapitreProps) -> Html {
    let chapitre = props.chapitre;

    let encadre = match &chapitre.encadre {
        Some(encadre) => html! {
            <aside class="encadre">
                <p class="encadre-etiquette">{ encadre.etiquette }</p>
                <p>{ encadre.texte }</p>
            </aside>
        },
        None => html! {},
    };

    html! {
        <article class="chapitre" id={format!("chapitre-{}", chapitre.numero)}>
            <p class="chapitre-repere">{ format!("Chapitre {} · {}", chapitre.numero, chapitre.etape) }</p>
            <h2>{ chapitre.titre }</h2>
            <figure class="chapitre-photo">
                <img src={chapitre.image} alt={chapitre.legende} width="1600" height="1000" loading="lazy" />
                <figcaption>{ chapitre.legende }</figcaption>
            </figure>
            { for chapitre.paragraphes.iter().map(|&paragraphe| html! { <p>{ paragraphe }</p> }) }
            { encadre }
        </article>
    }
}

#[component]
fn Comprendre() -> Html {
    html! {
        <section class="comprendre" id="comprendre">
            <div class="comprendre-int">
                <p class="surtitre">{ "Pour sensibiliser" }</p>
                <h2>{ "Comprendre la catégorie 2" }</h2>
                <p class="intro">
                    { "En France, la loi classe certains chiens en deux catégories. Eros appartient à la deuxième, celle des chiens de garde et de défense." }
                </p>
                <div class="comprendre-grille">
                    <div class="bloc">
                        <h3>{ "Quels chiens sont concernés ?" }</h3>
                        <p>{ CHIENS_CATEGORIE_2 }</p>
                        <h3>{ "Ce que la loi demande au maître" }</h3>
                        <dl class="obligations">
                            { for OBLIGATIONS.iter().map(|&(nom, detail)| html! {
                                <div class="obligation">
                                    <dt>{ nom }</dt>
                                    <dd>{ detail }</dd>
                                </div>
                            }) }
                        </dl>
                    </div>
                    <div class="bloc">
                        <h3>{ "L'évaluation comportementale" }</h3>
                        <p>{ "Un vétérinaire agréé observe le chien et le classe sur une échelle de 1 à 4. Eros a obtenu le niveau 1." }</p>
                        <ol class="echelle">
                            { for NIVEAUX.iter().map(|&(niveau, texte)| {
                                let est_eros = niveau == NIVEAU_EROS;
                                html! {
                                    <li class={classes!("niveau", est_eros.then_some("niveau-eros"))}>
                                        <span class="niveau-num">{ niveau }</span>
                                        <span class="niveau-texte">{ texte }</span>
                                        if est_eros {
                                            <span class="niveau-marque">{ "Eros" }</span>
                                        }
                                    </li>
                                }
                            }) }
                        </ol>
                    </div>
                </div>
                <div class="message">
                    <h3>{ "Ce que je voudrais que vous reteniez" }</h3>
                    <p>{ "La race ne dit pas tout d'un chien. Ce qu'il a vécu, son éducation et la façon dont on le traite comptent énormément. Eros a été maltraité, et il a quand même appris la confiance." }</p>
                    <p>{ "Si vous êtes témoin de maltraitance envers un animal, contactez la police, la gendarmerie ou une association de protection animale." }</p>
                </div>
                <p class="source">
                    { "Règles détaillées sur " }
                    <a href="https://www.service-public.gouv.fr/particuliers/vosdroits/F1839" target="_blank" rel="noopener">{ "service-public.gouv.fr" }</a>
                </p>
            </div>
        </section>
    }
}

#[component]
fn Nouvelles() -> Html {
    html! {
        <section class="nouvelles">
            <div class="nouvelles-int">
                <h2>{ "Les nouvelles d'Eros" }</h2>
                <p>{ "Bientôt ici : ses sorties, ses progrès et la suite de son histoire." }</p>
                <div class="galerie">
                    { for GALERIE.iter().map(|&(image, legende)| html! {
                        <figure class="galerie-photo">
                            <img src={image} alt={legende} width="900" height="1200" loading="lazy" />
                            <figcaption>{ legende }</figcaption>
                        </figure>
                    }) }
                </div>
            </div>
        </section>
    }
}

#[component]
fn PiedDePage() -> Html {
    html! {
        <footer class="pied">
            <div class="pied-int">
                <p>{ "Le blog d'Eros, tenu par Bruno." }</p>
            </div>
        </footer>
    }
}
