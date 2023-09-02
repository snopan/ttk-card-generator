<div>
    <div v-if="characterInfo">
        <div id="name-title">
            <div style="margin-right: 10px;">{{ characterInfo.name }}</div>
            <div v-if="characterInfo.male">:male_sign:</div>
            <div v-else>:female_sign:</div>
            <div v-if="characterInfo.monarch">:crown:</div>
        </div>
        <div class="field">
            Health: 
            <span v-for="index in characterInfo.health">:hearts:</span>
        </div>
        <div class="field">Kingdom: {{ characterInfo.kingdom.charAt(0).toUpperCase() + characterInfo.kingdom.slice(1) }}</div>
        <div id="skills-title">Skills</div>
        <div id="skills-list">
            <div class="skill" v-for="s in characterInfo.skills">
                <div class="skill-name">{{ s.name }}</div>
                <div class="skill-description">{{ s.description }}</div>
            </div>
        </div>
    </div>
    <div v-else>No character found</div>
</div>

<style>
    #name-title {
        font-size: 2rem;
        display: flex;
    }

    .field {
        font-size: 1.2rem;
        margin: 20px 0;
    }

    #skills-title {
        font-size: 1.7rem;
        margin: 20px 0;
    }

    #skills-list {
        width: 50%;
        border: 1px solid black;
        border-radius: 10px;
    }

    .skill {
        padding: 10px;
        border-bottom: 1px solid black
    }
    
    .skill:last-child {
        border-bottom: none
    }

    .skill-name {
        font-size: 1.5rem;
        font-weight: 400;
        margin-bottom: 10px;
    }

    .skill-description {
        font-size: 1.2rem;
    }

    #name-title, #skills-title {
        font-weight: 600;
    }

</style>