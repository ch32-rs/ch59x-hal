/* ch58x */
MEMORY
{
    /* Code Flash, 448KB max */
  FLASH : ORIGIN = 0x00000000, LENGTH = 448k
    /* SRAM, 26KB, 24KB + 2KB max */
  RAM : ORIGIN = 0x20000000, LENGTH = 26k
}

REGION_ALIAS("REGION_TEXT", FLASH);
REGION_ALIAS("REGION_RODATA", FLASH);
REGION_ALIAS("REGION_DATA", RAM);
REGION_ALIAS("REGION_BSS", RAM);
REGION_ALIAS("REGION_HEAP", RAM);
REGION_ALIAS("REGION_STACK", RAM);
