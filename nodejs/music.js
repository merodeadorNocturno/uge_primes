const durations = Object.assign({}, {ta: `♩`}, {ti_ti: `♫`}, {tiriTiri: `♬♬`}, {sh: `𝄽`}, {ti_sh: `𝅘𝅥𝅮  𝄾`}, {sh_ti: `𝄾 𝅘𝅥𝅮`});
console.log(durations);

const decompose = note => note.split(` `);

const setBpm = (bpm = 120) => bpm;
const setMeasure = (measure = 4) => measure;

const addFiguresToMeasure = (durations, measureParam = 4) => {
    const measure = setMeasure(measureParam);
    const measuredArray = new Array(measure).fill(0);
    const figures = Object.keys(durations);

    return measuredArray.map((i, j) => {
        let index = Math.round(Math.random() * figures.length - 1);
        index = (index >= 0) ? index : 0;
        return figures[index];
    });
};

const generateMeasures = (durations, numberOfMeasures = 1, measure = 4) => {
    const measures = new Array(numberOfMeasures).fill(0);
    return measures.map(i => addFiguresToMeasure(durations, measure));
};

const calculateBeatTime = _ => beatTime = 60000 / setBpm();

const breakNote = note => {
    const time = calculateBeatTime();
    const beatArray = note.split(` `);
    const timePerNote = time / beatArray.length;
    
    beatArray.map((beat, index) => setTimeout(() => {}, timePerNote));
};

const playMeasure = measure => {
    measure.map(figure => {breakNote(durations[figure])});
};

const playAll = measures => measures
    .map(measure => measure.map(note => durations[note]));
    // .map(item => item.map(note => durations[note]));


console.log(
  playAll(generateMeasures(durations, 4, 4))
);
