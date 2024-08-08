// SPDX-License-Identifier: GPL-2.0-only
/*
 * ALSA SoC SPDIF DIR (Digital Interface Reciever) driver
 *
 * Based on ALSA SoC SPDIF DIT driver
 *
 *  This driver is used by controllers which can operate in DIR (SPDI/F) where
 *  no codec is needed.  This file provides stub codec that can be used
 *  in these configurations. SPEAr SPDIF IN Audio controller uses this driver.
 *
 * Author:      Vipin Kumar,  <vipin.kumar@st.com>
 * Copyright:   (C) 2012  ST Microelectronics
 */

#include <linux/moduleparam.h>
#include <linux/slab.h>
#include <sound/soc.h>
#include <sound/pcm.h>
#include <sound/initval.h>
#include <linux/of.h>

static const struct snd_soc_dapm_widget dir_widgets[] = {
	SND_SOC_DAPM_INPUT("phoneodeo"),
};

static const struct snd_soc_dapm_route dir_routes[] = {
	{ "Capture", NULL, "phoneodeo" },
};

#define STUB_RATES	SNDRV_PCM_RATE_48000
#define STUB_FORMATS	(SNDRV_PCM_FMTBIT_S16_LE | \
			SNDRV_PCM_FMTBIT_S24_LE  | \
			SNDRV_PCM_FMTBIT_S32_LE | \
			SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE)

static struct snd_soc_component_driver soc_codec_phoneodeo_dir = {
	.dapm_widgets		= dir_widgets,
	.num_dapm_widgets	= ARRAY_SIZE(dir_widgets),
	.dapm_routes		= dir_routes,
	.num_dapm_routes	= ARRAY_SIZE(dir_routes),
	.idle_bias_on		= 1,
	.use_pmdown_time	= 1,
	.endianness		= 1,
};

static struct snd_soc_dai_driver dir_stub_dai = {
	.name		= "phoneodeo",
	.capture	= {
		.stream_name	= "Capture",
		.channels_min	= 1,
		.channels_max	= 2,
		.rates		= STUB_RATES,
		.formats	= STUB_FORMATS,
	},
};

static int phoneodeo_dir_probe(struct platform_device *pdev)
{
	return devm_snd_soc_register_component(&pdev->dev,
			&soc_codec_phoneodeo_dir,
			&dir_stub_dai, 1);
}

#ifdef CONFIG_OF
static const struct of_device_id phoneodeo_dir_dt_ids[] = {
	{ .compatible = "linux,phoneodeo-dir", },
	{ }
};
MODULE_DEVICE_TABLE(of, phoneodeo_dir_dt_ids);
#endif

static struct platform_driver phoneodeo_dir_driver = {
	.probe		= phoneodeo_dir_probe,
	.driver		= {
		.name	= "phoneodeo-dir",
		.of_match_table = of_match_ptr(phoneodeo_dir_dt_ids),
	},
};

module_platform_driver(phoneodeo_dir_driver);

MODULE_DESCRIPTION("Phoneodeo driver");
MODULE_AUTHOR("PETTEERETER <goapplyto@recurse.com>");
MODULE_LICENSE("GPL");
