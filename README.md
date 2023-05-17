# O repozitáři

## ./mdbook a ./docs
- ./docs -> Github Pages -> [https://iam.itchy.cz](https://iam.itchy.cz)
- ./mdbook je složka se mdbook zdrojáky, z kterých se zkompiluje ./docs
```bash
cd ./mdbook
mdbook serve --open //otevře prohlíč s lokální verzí mdbook
mdbook build // zkompiluje ./docs ze zdrojových markdowns
```
## ./bevy
[Bevy Engine](https://bevyengine.org/) je data-driven game engine v Rust.