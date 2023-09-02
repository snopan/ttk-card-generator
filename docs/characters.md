# Characters

<div id="characters-wrapper">
  <div id="search">
    <input
      type="search"
      placeholder="Type in the character name here..."
      v-model="search"
    />
    <div id="character-list">
      <div 
        class="character-items"
        v-for="c in characters"
        @click="clickCharacter(c.name)"
        v-show="!search.length || c.name.toLowerCase().includes(search)" 
      >
        <div :class="`kingdom ${c.kingdom} ${c.monarch && 'zhu'}`"></div>
        <div>{{ c.name }}</div>
      </div>
    </div>
  </div>
  <div></div>
  <div id="info">
    <div v-if="characterInfo">
      <div id="name-title">
        <div style="margin-right: 10px;">{{ characterInfo.name }}</div>
        <div v-if="characterInfo.male" title="Male">:male_sign:</div>
        <div v-else title="Female">:female_sign:</div>
        <div v-if="characterInfo.monarch" title="Monarch">:crown:</div>
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
    <div v-else>Please select a character</div>
  </div>
</div>

<style>
  #characters-wrapper {
    display: grid;
    height: 75vh;
    overflow: hidden;
    grid-template-columns: 38% 4% 58%;
  }

  #search, #info {
    width: 100%;
    min-height: 75%;
    padding: 10px;
    border: 1px solid black;
    border-radius: 10px;
  }

  #character-list {
    height: 90%;
    overflow-y: auto;
    border: 1px solid black;
  }

  .character-items {
    display: flex;
    align-items: center;
    cursor: pointer;
    padding: 15px;
    border-bottom: 1px solid black;
  }

  .character-items:hover {
    background-color: black;
  }

  .kingdom {
    width: 10px;
    height: 10px;
    margin-right: 10px;
  }

  .shu {
    background-color: red;
  }

  .wei {
    background-color: blue;
  }

  .wu {
    background-color: green;
  }
  
  .kingdomless {
    background-color: gray;
  }

  .zhu {
    border: 1px solid yellow;
  }

  #info {
    overflow-y: auto;
  }

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
    width: 100%;
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

  input {
    font-size:18px;
    padding:10px 10px 10px 5px;
    display:block;
    width:100%;
    border:none;
    border-bottom:1px solid #757575;
    border-radius: 10px;
    margin-bottom: 10px;
  }
  input:focus 		{ outline:none; }
</style>