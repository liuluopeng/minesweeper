<script setup lang="ts">

import { onMounted, reactive } from 'vue'

import { GameMap } from "my_wasm"

import { NButton, useMessage, createDiscreteApi } from "naive-ui"

import { ref, watch } from 'vue';
import { NInputNumber, NSwitch, NModal, NCard, NPopover } from "naive-ui";

// const message = useMessage()
const { message } = createDiscreteApi(["message"]);

/*
   > String0️⃣1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣
*/




const handleRightClickGrid = (e: { preventDefault: () => void; }, row: number, col: number) => {
  e.preventDefault()
  console.log('监听右键点击', row, col, e)


  game_map.flag_click(row, col);

  fresh_hover_map();
  data.flags = game_map.count_flag();




  if (!game_map.still_alive()) {
    message.error("失败 请重开")
    game_map.show_all_mine();
    fresh_hover_map();
  }


  if (game_map.can_I_win()) {
    message.success("成功 请重开")
  }
}

const data = reactive({
  timer: 0,
  flags: 0,
  hover_map: [] as string[],
  quickfind: ["0️⃣", "1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣", "6️⃣", "7️⃣", "8️⃣"],
  width: 30,
  height: 10,
  mine: 1,
  showModal: false,
  bury_list: [] as string[],
  diy_bury_mode: false,
  answer: [] as string[],
})

let game_map = GameMap.new(data.width, data.height, data.mine, true);
data.answer = game_map.get_answer();



function fresh_hover_map() {
  // 比对两个一维的list
  data.hover_map = game_map.get_hover_1d();

  console.log(data.hover_map)
}


function handleClickGrid(row_coo: number, col_coo: number) {
  // 如果打开: 什么也不做.
  if (game_map.still_alive()) {

    if (!game_map.opened(row_coo, col_coo)) {
      console.log("要打开", row_coo, col_coo);

      let open_list = game_map.foot_click(row_coo, col_coo);

      console.log(open_list);

      // 刷新
      fresh_hover_map();

    }

  }



  if (!game_map.still_alive()) {
    message.error("失败 请重开")
    game_map.show_all_mine();
    fresh_hover_map();
  }


  if (game_map.can_I_win()) {
    message.success("成功 请重开")
  }
}
function handleDbClickGrid(row_coo: number, col_coo: number) {
  // console.log("双击打开", row_coo, col_coo);

  // 如果还没有打开: 什么也不做.

  // if (game_map.still_alive()) {

  if (game_map.opened(row_coo, col_coo)) {
    console.log("opened 双击", row_coo, col_coo);

    let msg = game_map.double_click(row_coo, col_coo);
    // let msg = "ss";
    fresh_hover_map();
    if (msg == null) {
    } else {
      message.info(msg);
    }

  }
  // }



  if (!game_map.still_alive()) {
    message.error("失败 请重开")
    game_map.show_all_mine();
    fresh_hover_map();
  }

  if (game_map.can_I_win()) {
    message.success("成功 请重开")
  }
}









onMounted(() => {
  let size = data.width * data.height;
  data.bury_list = []
  for (var i = 0; i < size; i++) {
    data.bury_list.push("🈳")
  }


  console.log(`the component is now mounted.`)
  console.log(game_map.peek());

  fresh_hover_map();
})




// computed: update bury list length 


const handleClickBuryMine = (row: number, col: number) => {

  let index = data.width * row + col;

  if (data.bury_list[index] === "💣") {
    data.bury_list[index] = "🈳"
  } else {
    data.bury_list[index] = "💣"
  }

  // data.bury_list[index] == "💣" ? "🈳" : "💣";



}


const handleSetting = () => {
  data.showModal = true
}


const seeAnswer = () => {

}


const newGame = () => {
  // new
  game_map = GameMap.new(data.width, data.height, data.mine, true);
  data.answer = game_map.get_answer();

  fresh_hover_map()
}


const handelLoadSetting = () => {

  data.showModal = false

  // 确定 后
  if (data.diy_bury_mode) {
    // new1 
    let mine_1d = [];
    for (var i = 0; i < data.bury_list.length; i++) {
      if (data.bury_list[i] === "🈳") {
        mine_1d.push(0)
      } else {
        mine_1d.push(1)
      }
    }
    const u32_mine_1d = new Uint32Array(mine_1d);

    game_map = GameMap.new2(data.width, data.height, u32_mine_1d)
    fresh_hover_map()
    data.answer = game_map.get_answer();

  } else {

    // new
    game_map = GameMap.new(data.width, data.height, data.mine, true);
    fresh_hover_map()
    data.answer = game_map.get_answer();

  }
}


watch(() => data.width, (newWidth,) => {
  let size = newWidth * data.height;
  data.bury_list = []
  for (var i = 0; i < size; i++) {
    data.bury_list.push("🈳")
  }

})

watch(() => data.height, (newHeight,) => {
  let size = newHeight * data.width;
  data.bury_list = []
  for (var i = 0; i < size; i++) {
    data.bury_list.push("🈳")
  }
})


</script>

<template>





  <br />

  <n-modal v-model:show="data.showModal">

    <n-card>
      <div>
        地图宽度: 
        <n-input-number v-model:value="data.width" />
        地图高度:
        <n-input-number v-model:value="data.height" />



        规定地雷数量, 或者手动埋雷.

        <n-switch v-model:value="data.diy_bury_mode">
          <template #checked>
            手动埋雷
          </template>
          <template #unchecked>
            规定地雷数量
          </template>
        </n-switch>

        <br />

        <div v-show="!data.diy_bury_mode">
          地雷数量:
          <n-input-number v-model:value="data.mine" />
        </div>





        手动埋雷:

        <div v-show="data.diy_bury_mode">

          <div class="row_class" v-for="i, row in data.height" :key="row">
            <div v-for="j, col in data.width" :key="col" @click="handleClickBuryMine(row, col)">

              {{ data.bury_list[row * data.width + col] }}
            </div>
          </div>

        </div>



        <n-button @click="handelLoadSetting()">
          确定游戏设置
        </n-button>


      </div>
    </n-card>
  </n-modal>










  <div>

    <!-- <pre> {{ game_map.peek() }} </pre> -->


    <div>







      <n-popover placement="bottom" trigger="click">
        <template #trigger>
          <n-button @click="seeAnswer()"> 看答案 </n-button>
        </template>
        <div class=" row_class" v-for="(row, row_coo) in data.height" :key="row_coo">
          <div v-for="col, col_coo in data.width" :key="col_coo">

            {{ data.answer[row_coo * data.width + col_coo] }}

          </div>
        </div>

      </n-popover>

      <n-popover placement="bottom" trigger="click">
        <template #trigger>
          <n-button>游戏玩法</n-button>
        </template>
        <div class=" row_class">
          左键: 排雷 <br />
          对着数字双击: 快速排空周围8个方块 <br />
          右键: 标记地雷<br />
        </div>

      </n-popover>

      <n-button @click="newGame()">新游戏 </n-button>
      <n-button @click="handleSetting()">
        设置游戏
      </n-button>


      {{ data.flags }} 🚩
    </div>

    <div class="row_class" v-for="(row, row_coo) in data.height" :key="row_coo">


      <div class="grid_item_class" v-for="(col, col_coo) in data.width" :key="col_coo">


        <n-button circle @click="handleClickGrid(row_coo, col_coo)" @dblclick="handleDbClickGrid(row_coo, col_coo)"
          @click.right.native=" handleRightClickGrid($event, row_coo, col_coo)">


          {{ data.hover_map[row_coo * data.width + col_coo] }}
        </n-button>

        <!-- <div 
                    @click="handleClickGrid(row_coo, col_coo)" 
                    @dblclick="handleDbClickGrid(row_coo, col_coo)"
                    @click.right.native=" handleRightClickGrid($event, row_coo, col_coo)"
                >

                    {{ data.hover_map[row_coo * data.width + col_coo] }}
                </div> -->



      </div>

    </div>





  </div>

</template>

<style scoped>
header {
  line-height: 1.5;
}

.logo {
  display: block;
  margin: 0 auto 2rem;
}

@media (min-width: 1024px) {
  header {
    display: flex;
    place-items: center;
    padding-right: calc(var(--section-gap) / 2);
  }

  .logo {
    margin: 0 2rem 0 0;
  }

  header .wrapper {
    display: flex;
    place-items: flex-start;
    flex-wrap: wrap;
  }



}


.my_map {
  border: 1px solid;
  background-color: aliceblue;
}



.row_class {
  display: flex;
}
</style>
