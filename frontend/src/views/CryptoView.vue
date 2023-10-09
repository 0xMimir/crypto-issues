<script setup lang="ts">
import { useRoute } from 'vue-router';

const cryptoId = useRoute().params.id;

</script>
<template>
    <h1>
        {{ cryptoId }}
    </h1>
</template>