document.addEventListener('DOMContentLoaded', function () {
    let starter = document.getElementById('starter');
    let quarter = document.getElementById('quarter');
    let min = document.getElementById('min');
    let minRange1 = document.getElementById('minRange1');
    let minRange100 = document.getElementById('minRange100');
    let size = document.getElementById('size');
    let sizeRange = document.getElementById('sizeRange');
    let fraction = document.getElementById('fraction');
    let fractionRange = document.getElementById('fractionRange');
    let precision = document.getElementById('precision');
    let precisionRange = document.getElementById('precisionRange');
    let step = document.getElementById('step');
    let stepRange = document.getElementById('stepRange');
    let preview = document.getElementById('preview');
    let blueprint = document.getElementById('blueprint');
    let note = document.getElementById('note');

    /**
     * @param {HTMLInputElement} number
     * @param {HTMLInputElement} range
     */
    function onNumberChange(number, range) {
        let value = parseInt(number.value, 10);
        let min = parseInt(range.min, 10);
        let max = parseInt(range.max, 10);

        if (value < min) {
            range.value = min.toString(10);
            return;
        }

        if (value < max) {
            range.value = value.toString(10);
            return;
        }

        range.value = max.toString(10);
    }

    /**
     * @param {HTMLInputElement} number
     * @param {HTMLInputElement} range
     */
    function onRangeChange(number, range) {
        number.value = range.value;
    }

    /**
     * @param {HTMLInputElement} number
     * @param {HTMLInputElement} range
     */
    function registerRangeChange(number, range) {
        range.addEventListener('input', () => onRangeChange(number, range));
        range.addEventListener('input', render);
        number.addEventListener('input', () => onNumberChange(number, range));
        number.addEventListener('input', render);
        onNumberChange(number, range);
    }

    function onMinRangeChange() {
        min.value = (parseInt(minRange100.value, 10) * 100 + parseInt(minRange1.value, 10)).toString(10);
    }

    minRange1.addEventListener('input', onMinRangeChange);
    minRange1.addEventListener('input', render);
    minRange100.addEventListener('input', onMinRangeChange);
    minRange100.addEventListener('input', render);

    function onMinChange() {
        let value = parseInt(min.value, 10);
        minRange1.value = (((value - 1) % 100) + 1).toString(10);
        minRange100.value = Math.floor(value / 100).toString(10);
    }

    min.addEventListener('input', onMinChange);
    min.addEventListener('input', render);

    onMinChange();

    registerRangeChange(size, sizeRange);
    registerRangeChange(fraction, fractionRange);
    registerRangeChange(precision, precisionRange);
    registerRangeChange(step, stepRange);

    function updateFractionMax() {
        let max = parseInt(precision.value, 10);
        fraction.max = max.toString(10);
        fractionRange.max = max.toString(10);
        let fractionValue = parseInt(fraction.value, 10);
        if (max < fractionValue) {
            fraction.value = max.toString(10);
        }
        let fractionRangeValue = parseInt(fractionRange.value, 10);
        if (max < fractionRangeValue) {
            fractionRange.value = max.toString(10);
        }
    }

    precision.addEventListener('input', updateFractionMax);
    precisionRange.addEventListener('input', updateFractionMax);

    updateFractionMax();

    quarter.addEventListener('change', render);

    function enableRendering() {
        window.factorio_circle_generator_interacted = true;
        starter.disabled = true;
        note.classList.remove('hidden');
        render();
    }

    starter.addEventListener('click', enableRendering);

    function render() {
        if (!window.factorio_circle_generator_interacted) {
            return;
        }
        let $quarter = quarter.checked;
        let $min = parseInt(min.value, 10);
        let $size = parseInt(size.value, 10);
        let $fraction = parseInt(fraction.value, 10);
        let $precision = parseInt(precision.value, 10);
        let $step = parseInt(step.value, 10);
        let maxDiameter = ($min + $size - 1);
        let scale = 4;
        preview.width = (maxDiameter * scale).toString(10);
        preview.height = (maxDiameter * scale).toString(10);
        let ctx = preview.getContext('2d');
        if (!window.factorio_circle_generator_blueprint) {
            ctx.fillStyle = "#FF0088";
            ctx.fillRect(0, 0, (maxDiameter * scale), (maxDiameter * scale));
            return;
        }
        if (!window.factorio_circle_generator_preview) {
            ctx.fillStyle = "#FF8800";
            ctx.fillRect(0, 0, (maxDiameter * scale), (maxDiameter * scale));
            return;
        }
        ctx.fillStyle = "#FFFFFF";
        ctx.fillRect(0, 0, maxDiameter * scale, maxDiameter * scale);
        ctx.fillStyle = "#333333";
        let data = window.factorio_circle_generator_preview($quarter, $min, maxDiameter, $fraction, $precision, $step);
        let index = 0;
        let length = data.length;
        while (index < length) {
            let diameter = data[index];
            ++index;
            let count = data[index];
            ++index;
            let offset = Math.floor(diameter / 2);
            if (1 === (diameter % 2)) {
                ++offset;
            }
            offset += Math.floor((maxDiameter - diameter) / 2);
            for (let i = 0; i < count; ++i) {
                let x = data[index];
                ++index;
                let y = data[index];
                ++index;
                ctx.fillRect((x + offset) * scale, (y + offset) * scale, scale, scale);
            }
        }

        let blueprintString = window.factorio_circle_generator_blueprint($quarter, $min, maxDiameter, $fraction, $precision, $step);
        blueprint.value = blueprintString;
        navigator.clipboard.writeText(blueprintString);
    }

    render();

    window.factorio_circle_generator_render_preview = render;
});

import init, { generate_blueprint, generate_points } from './rust/factorio_circle_generator.js';

await init();

window.factorio_circle_generator_blueprint = generate_blueprint;
window.factorio_circle_generator_preview = generate_points;
window.factorio_circle_generator_interacted = false;

if (window.factorio_circle_generator_render_preview) {
    window.factorio_circle_generator_render_preview();
}
