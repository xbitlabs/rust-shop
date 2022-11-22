/*
 Navicat Premium Data Transfer

 Source Server         : local
 Source Server Type    : MySQL
 Source Server Version : 80027
 Source Host           : localhost:3306
 Source Schema         : rust_shop

 Target Server Type    : MySQL
 Target Server Version : 80027
 File Encoding         : 65001

 Date: 02/11/2022 18:25:37
*/

SET NAMES utf8mb4;
SET
FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for admin_permission
-- ----------------------------
DROP TABLE IF EXISTS `admin_permission`;
CREATE TABLE `admin_permission`
(
    `id`                        bigint NOT NULL,
    `admin_permission_group_id` bigint NULL DEFAULT NULL,
    `title`                     varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
    `code`                      varchar(30) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
    `url`                       varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
    PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of admin_permission
-- ----------------------------

-- ----------------------------
-- Table structure for admin_permission_group
-- ----------------------------
DROP TABLE IF EXISTS `admin_permission_group`;
CREATE TABLE `admin_permission_group`
(
    `id`        bigint NOT NULL,
    `name`      varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
    `parent_id` bigint NULL DEFAULT NULL,
    PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of admin_permission_group
-- ----------------------------

-- ----------------------------
-- Table structure for admin_role
-- ----------------------------
DROP TABLE IF EXISTS `admin_role`;
CREATE TABLE `admin_role`
(
    `id`          bigint NOT NULL,
    `name`        varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
    `description` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
    PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of admin_role
-- ----------------------------

-- ----------------------------
-- Table structure for admin_role_permission
-- ----------------------------
DROP TABLE IF EXISTS `admin_role_permission`;
CREATE TABLE `admin_role_permission`
(
    `admin_role_id`       bigint NOT NULL,
    `admin_permission_id` bigint NOT NULL,
    PRIMARY KEY (`admin_role_id`, `admin_permission_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of admin_role_permission
-- ----------------------------

-- ----------------------------
-- Table structure for admin_user
-- ----------------------------
DROP TABLE IF EXISTS `admin_user`;
CREATE TABLE `admin_user`
(
    `id`       bigint NOT NULL,
    `username` varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
    `password` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
    PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of admin_user
-- ----------------------------

-- ----------------------------
-- Table structure for admin_user_role
-- ----------------------------
DROP TABLE IF EXISTS `admin_user_role`;
CREATE TABLE `admin_user_role`
(
    `admin_user_id` bigint NOT NULL,
    `admin_role_id` bigint NOT NULL,
    PRIMARY KEY (`admin_user_id`, `admin_role_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of admin_user_role
-- ----------------------------

-- ----------------------------
-- Table structure for discount_coupon
-- ----------------------------
DROP TABLE IF EXISTS `discount_coupon`;
CREATE TABLE `discount_coupon`
(
    `id` bigint NOT NULL,
    PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of discount_coupon
-- ----------------------------

-- ----------------------------
-- Table structure for order
-- ----------------------------
DROP TABLE IF EXISTS `order`;
CREATE TABLE `order`
(
    `id`                  bigint                                                        NOT NULL,
    `user_id`             bigint                                                        NOT NULL,
    `logistics_status`    varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
    `pay_status`          varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL,
    `recipient`           varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL,
    `phone_number`        varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci  NOT NULL,
    `address`             varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL,
    `post_code`           varchar(10) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci  NOT NULL,
    `is_pick_up_in_store` bit(1)                                                        NOT NULL,
    `pick_up_code`        varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
    `is_pick_up_already`  bit(1) NULL DEFAULT NULL,
    `pick_up_time`        datetime NULL DEFAULT NULL,
    `remark`              varchar(1000) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
    `created_time`        datetime                                                      NOT NULL,
    PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of order
-- ----------------------------

-- ----------------------------
-- Table structure for order_item
-- ----------------------------
DROP TABLE IF EXISTS `order_item`;
CREATE TABLE `order_item`
(
    `id`         bigint         NOT NULL,
    `order_id`   bigint         NOT NULL,
    `product_id` bigint         NOT NULL,
    `sku_id`     bigint         NOT NULL,
    `quantity`   int            NOT NULL,
    `price`      decimal(10, 2) NOT NULL,
    PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of order_item
-- ----------------------------

-- ----------------------------
-- Table structure for pay_log
-- ----------------------------
DROP TABLE IF EXISTS `pay_log`;
CREATE TABLE `pay_log`
(
    `id`               bigint   NOT NULL,
    `order_id`         bigint   NOT NULL,
    `pay_request_info` text CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL,
    `pay_response`     text CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL,
    `callback_infos`   text CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL,
    `pay_time`         datetime NOT NULL,
    PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of pay_log
-- ----------------------------

-- ----------------------------
-- Table structure for product
-- ----------------------------
DROP TABLE IF EXISTS `product`;
CREATE TABLE `product`
(
    `id`                 bigint                                                         NOT NULL,
    `name`               varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci  NOT NULL,
    `cover_image`        varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci  NOT NULL,
    `category_id`        bigint                                                         NOT NULL,
    `pics_and_video`     varchar(2000) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL,
    `description`        text CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL,
    `status`             varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci  NOT NULL,
    `created_time`       datetime                                                       NOT NULL,
    `last_modified_time` datetime NULL DEFAULT NULL,
    PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of product
-- ----------------------------

-- ----------------------------
-- Table structure for product_category
-- ----------------------------
DROP TABLE IF EXISTS `product_category`;
CREATE TABLE `product_category`
(
    `id`         bigint                                                       NOT NULL,
    `name`       varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL,
    `icon`       varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
    `pic`        varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
    `sort_index` int                                                          NOT NULL,
    PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of product_category
-- ----------------------------
INSERT INTO `product_category`
VALUES (1, '男士上衣', NULL, NULL, 1);

-- ----------------------------
-- Table structure for promotion
-- ----------------------------
DROP TABLE IF EXISTS `promotion`;
CREATE TABLE `promotion`
(
    `id` bigint NOT NULL,
    PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of promotion
-- ----------------------------

-- ----------------------------
-- Table structure for role
-- ----------------------------
DROP TABLE IF EXISTS `role`;
CREATE TABLE `role`
(
    `id`          bigint NOT NULL,
    `name`        varchar(30) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
    `description` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
    PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of role
-- ----------------------------

-- ----------------------------
-- Table structure for shopping_cart
-- ----------------------------
DROP TABLE IF EXISTS `shopping_cart`;
CREATE TABLE `shopping_cart`
(
    `id`         bigint   NOT NULL,
    `product_id` bigint   NOT NULL,
    `sku_id`     bigint   NOT NULL,
    `quantity`   int      NOT NULL,
    `user_id`    bigint   NOT NULL,
    `add_time`   datetime NOT NULL,
    PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of shopping_cart
-- ----------------------------

-- ----------------------------
-- Table structure for sku
-- ----------------------------
DROP TABLE IF EXISTS `sku`;
CREATE TABLE `sku`
(
    `id`         bigint                                                        NOT NULL,
    `title`      varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL,
    `product_id` bigint                                                        NOT NULL,
    `price`      decimal(10, 2)                                                NOT NULL,
    `is_default` bit(1)                                                        NOT NULL,
    PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sku
-- ----------------------------

-- ----------------------------
-- Table structure for user
-- ----------------------------
DROP TABLE IF EXISTS `user`;
CREATE TABLE `user`
(
    `id`                       bigint   NOT NULL,
    `username`                 varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
    `password`                 varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
    `phone_number`             varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
    `is_phone_number_verified` bit(1) NULL DEFAULT NULL,
    `wx_open_id`               varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
    `enable`                   bit(1)   NOT NULL DEFAULT b'1',
    `created_time`             datetime NOT NULL,
    PRIMARY KEY (`id`) USING BTREE,
    UNIQUE INDEX `wx_open_id`(`wx_open_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of user
-- ----------------------------
INSERT INTO `user`
VALUES (1, 'test', '$2b$12$WjTkxnMfCMG4wPijgXqUn.aRVYk0D5Mk6qQmrLW6OOrWE.fllf4ki', NULL, b'0', '333', b'1',
        '2022-11-02 12:26:52');

-- ----------------------------
-- Table structure for user_jwt
-- ----------------------------
DROP TABLE IF EXISTS `user_jwt`;
CREATE TABLE `user_jwt`
(
    `id`            bigint                                                         NOT NULL,
    `user_id`       bigint                                                         NOT NULL,
    `token_id`      varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci   NOT NULL,
    `access_token`  varchar(2000) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL,
    `refresh_token` varchar(2000) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL,
    `issue_time`    datetime                                                       NOT NULL,
    PRIMARY KEY (`id`) USING BTREE,
    INDEX           `user_id_token_id`(`user_id`, `token_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of user_jwt
-- ----------------------------
INSERT INTO `user_jwt`
VALUES (6981529611242967041, 1111, '7c108a3a-fe74-4063-ae5d-b2fba47d807b',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjdjMTA4YTNhLWZlNzQtNDA2My1hZTVkLWIyZmJhNDdkODA3YiIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUyNjM2OSwiZXhwIjoxNjY1MTMxMTY5fQ.Qhe_DHkI0ZL10prnE0moE1Oejm8cGGGtvoRINtNAe7w',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjdjMTA4YTNhLWZlNzQtNDA2My1hZTVkLWIyZmJhNDdkODA3YiIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUyNjM2OSwiZXhwIjoxNjY1ODIyMzY5fQ.jo1f8BSCjn3Y_h1wo4JSjbfVegEPuFInoa5Sltf7Hbw',
        '2022-09-30 08:26:09');
INSERT INTO `user_jwt`
VALUES (6981531341682774017, 1111, '0a696744-abba-4cb8-92ca-6c181a627c62',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjBhNjk2NzQ0LWFiYmEtNGNiOC05MmNhLTZjMTgxYTYyN2M2MiIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUyNjc4MiwiZXhwIjoxNjY1MTMxNTgyfQ.nxT6Kz-RGRAb6QcoFkW1y6df77xj5wpVBygKRWcDm9s',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjBhNjk2NzQ0LWFiYmEtNGNiOC05MmNhLTZjMTgxYTYyN2M2MiIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUyNjc4MiwiZXhwIjoxNjY1ODIyNzgyfQ.9Ne3C2-eTtZrb7mWEPPNrZriUcW9f1gRXtBoCbUfaMw',
        '2022-09-30 08:33:02');
INSERT INTO `user_jwt`
VALUES (6981531930353340417, 1111, '5b2e850b-e52f-448a-b4b2-c535ff336a79',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjViMmU4NTBiLWU1MmYtNDQ4YS1iNGIyLWM1MzVmZjMzNmE3OSIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUyNjkyMiwiZXhwIjoxNjY1MTMxNzIyfQ.Rl7jV3Sy1M3qUQUSSdBNz59K_DEGtqsZ5pXpOhYanU8',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjViMmU4NTBiLWU1MmYtNDQ4YS1iNGIyLWM1MzVmZjMzNmE3OSIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUyNjkyMiwiZXhwIjoxNjY1ODIyOTIyfQ.9mQI_DR_aKebbyhsvm_LW-7NRbWFzRrwFpUjjf98PAk',
        '2022-09-30 08:35:22');
INSERT INTO `user_jwt`
VALUES (6981532031834525697, 1111, 'a91991a1-130a-4866-a097-e2f8716c27f5',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImE5MTk5MWExLTEzMGEtNDg2Ni1hMDk3LWUyZjg3MTZjMjdmNSIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUyNjk0NiwiZXhwIjoxNjY1MTMxNzQ2fQ.shGR5ma6rsFDLwy51Q0KO7YCGIwMV2HC-QKESceZ3C4',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImE5MTk5MWExLTEzMGEtNDg2Ni1hMDk3LWUyZjg3MTZjMjdmNSIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUyNjk0NiwiZXhwIjoxNjY1ODIyOTQ2fQ.GlMZzK5bxMqOaIMd3P51-_WU7rt6IwPVPPpdHIHvUYI',
        '2022-09-30 08:35:46');
INSERT INTO `user_jwt`
VALUES (6981532958847012865, 1111, 'a44ba95f-fc73-4326-acf1-545d6f7c1d0d',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImE0NGJhOTVmLWZjNzMtNDMyNi1hY2YxLTU0NWQ2ZjdjMWQwZCIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MSwiZXhwIjo2MDQ4MDF9.v9cLLOjTQueavYVwQDLhJhZwB2OGyFdERcyoAK1Ggr8',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImE0NGJhOTVmLWZjNzMtNDMyNi1hY2YxLTU0NWQ2ZjdjMWQwZCIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MSwiZXhwIjoxMjk2MDAxfQ.gIM45ZzTRCv9BRfw843cVlLcKCJ2M1-Oh2gBzNQrUjk',
        '1970-01-01 00:00:01');
INSERT INTO `user_jwt`
VALUES (6981533566782017537, 1111, '9d164de5-16a8-4e66-905f-fcef06783d76',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjlkMTY0ZGU1LTE2YTgtNGU2Ni05MDVmLWZjZWYwNjc4M2Q3NiIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUyNzMxMiwiZXhwIjoxNjY1MTMyMTEyfQ.c4E3bOYHqB6jFuXpglWjN7k1QMi7qhDNRlvz4EJJpz4',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjlkMTY0ZGU1LTE2YTgtNGU2Ni05MDVmLWZjZWYwNjc4M2Q3NiIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUyNzMxMiwiZXhwIjoxNjY1ODIzMzEyfQ.HS0RbNAxFoDmsr5vsOmdBxAjlZ09SBS74i9fki8lXbM',
        '2022-09-30 08:41:52');
INSERT INTO `user_jwt`
VALUES (6981535660503076865, 1111, '009641d3-aa7a-4ac9-83c7-7f011e8ebb28',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjAwOTY0MWQzLWFhN2EtNGFjOS04M2M3LTdmMDExZThlYmIyOCIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUyNzgxMiwiZXhwIjoxNjY1MTMyNjEyfQ.iNgHDyOLt-Tu6mj2tSb2BHvwyxtApiKAisavCNeeoeQ',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjAwOTY0MWQzLWFhN2EtNGFjOS04M2M3LTdmMDExZThlYmIyOCIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUyNzgxMiwiZXhwIjoxNjY1ODIzODEyfQ.gGL9w_ad1BnnA7Cdbq5LDggQydBF3Wibx-KUCZNhl1E',
        '2022-09-30 08:50:12');
INSERT INTO `user_jwt`
VALUES (6981535660565991424, 1111, '3b04da48-bcdf-4e33-8742-fe4bea1f7073',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjNiMDRkYTQ4LWJjZGYtNGUzMy04NzQyLWZlNGJlYTFmNzA3MyIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUyNzgxMiwiZXhwIjoxNjY1MTMyNjEyfQ.bWJXoSVaL3LwU2oqlCbgnaf0nSIZwH4Q4UBTp4qqyw4',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjNiMDRkYTQ4LWJjZGYtNGUzMy04NzQyLWZlNGJlYTFmNzA3MyIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUyNzgxMiwiZXhwIjoxNjY1ODIzODEyfQ.AXlrdFeM6fXAcnF_7cxbRwuHIgtfzRRqBBUrAT6CLHw',
        '2022-09-30 08:50:12');
INSERT INTO `user_jwt`
VALUES (6981536005266477057, 1111, 'f9c4be5e-6b23-49bf-a356-5bbf596f3d7f',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImY5YzRiZTVlLTZiMjMtNDliZi1hMzU2LTViYmY1OTZmM2Q3ZiIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUyNzg5NCwiZXhwIjoxNjY1MTMyNjk0fQ.Yag1DQ0AXrKV9fYDvSqJaYOVWjGjP6dGs_hhbLx4gzs',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImY5YzRiZTVlLTZiMjMtNDliZi1hMzU2LTViYmY1OTZmM2Q3ZiIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUyNzg5NCwiZXhwIjoxNjY1ODIzODk0fQ.fCYELXLLQfTWI_Sg8O_I5G14DIE30myaSP8dsIz8u4c',
        '2022-09-30 08:51:34');
INSERT INTO `user_jwt`
VALUES (6981536005337780224, 1111, 'fcdf59c6-a2f3-476f-8ff6-6adcfab7f2b7',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImZjZGY1OWM2LWEyZjMtNDc2Zi04ZmY2LTZhZGNmYWI3ZjJiNyIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUyNzg5NCwiZXhwIjoxNjY1MTMyNjk0fQ.EaYs7wwbbD9G5mIvlLvr5KM82MlIpxCYpzDIQT3spsk',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImZjZGY1OWM2LWEyZjMtNDc2Zi04ZmY2LTZhZGNmYWI3ZjJiNyIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUyNzg5NCwiZXhwIjoxNjY1ODIzODk0fQ.G9Od-bSmFrnvcWVWy4OzuORE6J_4DRDe1nq2RDX9zL0',
        '2022-09-30 08:51:34');
INSERT INTO `user_jwt`
VALUES (6981536198787469313, 1111, 'cd83e985-6f19-43fe-8681-aae9440209e0',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImNkODNlOTg1LTZmMTktNDNmZS04NjgxLWFhZTk0NDAyMDllMCIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUyNzk0MCwiZXhwIjoxNjY1MTMyNzQwfQ.E_3p0pM4SNeBnBevE_foCsHJ32ZUCkylXD5ri1UuKFk',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImNkODNlOTg1LTZmMTktNDNmZS04NjgxLWFhZTk0NDAyMDllMCIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUyNzk0MCwiZXhwIjoxNjY1ODIzOTQwfQ.Vn9JIB0K7PKXxPwpX_tqkizzU9s0iHasBPfcpLJTf4k',
        '2022-09-30 08:52:20');
INSERT INTO `user_jwt`
VALUES (6981536198867161088, 1111, '76d52ac9-b98f-4253-b203-531e7d186d89',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6Ijc2ZDUyYWM5LWI5OGYtNDI1My1iMjAzLTUzMWU3ZDE4NmQ4OSIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUyNzk0MCwiZXhwIjoxNjY1MTMyNzQwfQ.AttJl1W0AxKTKl9OyNS6VHBKVCuns4nZdKpt8Ne6EVk',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6Ijc2ZDUyYWM5LWI5OGYtNDI1My1iMjAzLTUzMWU3ZDE4NmQ4OSIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUyNzk0MCwiZXhwIjoxNjY1ODIzOTQwfQ.EUDgM_h5fJm0qE7tQLj2zQEWCM1YwKNt_iKM7tANJac',
        '2022-09-30 08:52:20');
INSERT INTO `user_jwt`
VALUES (6981536294371463169, 1111, '84ef5e32-d636-4889-8e20-4bcfc980ac10',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6Ijg0ZWY1ZTMyLWQ2MzYtNDg4OS04ZTIwLTRiY2ZjOTgwYWMxMCIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUyNzk2MywiZXhwIjoxNjY1MTMyNzYzfQ.eqhecN5DeulLSqcvsXfXmpUMEJU60zfI3kUa_oMH5Ls',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6Ijg0ZWY1ZTMyLWQ2MzYtNDg4OS04ZTIwLTRiY2ZjOTgwYWMxMCIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUyNzk2MywiZXhwIjoxNjY1ODIzOTYzfQ.JT_cMLPXg7O61xYXjTgmgrMNJwW62s9tDJMcQp2FxKE',
        '2022-09-30 08:52:43');
INSERT INTO `user_jwt`
VALUES (6981536294413406208, 1111, '7ff65a08-95f9-49be-9773-e7b22aafb6be',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjdmZjY1YTA4LTk1ZjktNDliZS05NzczLWU3YjIyYWFmYjZiZSIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUyNzk2MywiZXhwIjoxNjY1MTMyNzYzfQ.YfcYejuHHRWVHU5XFZh24MkKQBLDEvdLkdEBmTsPjUY',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjdmZjY1YTA4LTk1ZjktNDliZS05NzczLWU3YjIyYWFmYjZiZSIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUyNzk2MywiZXhwIjoxNjY1ODIzOTYzfQ.36cZT04SrO7agSANQfLy8MxCmdtz8MjE4VwTcGnHptE',
        '2022-09-30 08:52:43');
INSERT INTO `user_jwt`
VALUES (6981536468921618433, 1111, 'c9f55b5c-bf8e-4caa-a663-48411ec467a4',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImM5ZjU1YjVjLWJmOGUtNGNhYS1hNjYzLTQ4NDExZWM0NjdhNCIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUyODAwNCwiZXhwIjoxNjY1MTMyODA0fQ.IKFAYeEmOC7Yunyfsb2wbmvtmvkh9o_GQ9b7jP6dPZA',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImM5ZjU1YjVjLWJmOGUtNGNhYS1hNjYzLTQ4NDExZWM0NjdhNCIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUyODAwNCwiZXhwIjoxNjY1ODI0MDA0fQ.nYOfQIwMGHV7ZiPPHvWYkg5hQ0R4aN0Wi8JT5_0orPk',
        '2022-09-30 08:53:24');
INSERT INTO `user_jwt`
VALUES (6981536468984532992, 1111, '4c509ce0-6cea-47bd-baad-e64a236110e3',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjRjNTA5Y2UwLTZjZWEtNDdiZC1iYWFkLWU2NGEyMzYxMTBlMyIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUyODAwNCwiZXhwIjoxNjY1MTMyODA0fQ.rK3IwV7lSDH7UzO3Tfq31grEvaxe7QH4rQIzGRjew5A',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjRjNTA5Y2UwLTZjZWEtNDdiZC1iYWFkLWU2NGEyMzYxMTBlMyIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUyODAwNCwiZXhwIjoxNjY1ODI0MDA0fQ.1cBPRknpUdl3vpFiqnjtFPFZ0uAFqGE4lZom-ArBB8w',
        '2022-09-30 08:53:24');
INSERT INTO `user_jwt`
VALUES (6981536744734855169, 1111, 'b3ade3c9-c0dd-45af-99d3-9f5d65fea180',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImIzYWRlM2M5LWMwZGQtNDVhZi05OWQzLTlmNWQ2NWZlYTE4MCIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUyODA3MCwiZXhwIjoxNjY1MTMyODcwfQ.OjS2scaNenFo4uLQUX62s1k-0g0nVwNHMMlo88mLilE',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImIzYWRlM2M5LWMwZGQtNDVhZi05OWQzLTlmNWQ2NWZlYTE4MCIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUyODA3MCwiZXhwIjoxNjY1ODI0MDcwfQ.OtReV-HSIPPMhcMibbNx6vh0acDT_k51HxTpuV39eb4',
        '2022-09-30 08:54:30');
INSERT INTO `user_jwt`
VALUES (6981536840432095233, 1111, 'fa020e3f-df8b-45f6-a4e1-20409abcd58a',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImZhMDIwZTNmLWRmOGItNDVmNi1hNGUxLTIwNDA5YWJjZDU4YSIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUyODA5MywiZXhwIjoxNjY1MTMyODkzfQ.8cHRzDVqkDkq7WLXVLLYEBgC6xbIdts7nBVaEm_wQG8',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImZhMDIwZTNmLWRmOGItNDVmNi1hNGUxLTIwNDA5YWJjZDU4YSIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUyODA5MywiZXhwIjoxNjY1ODI0MDkzfQ.BoEjodLFKafb47cONlyrMyZMdK4jEom5Gqb2PuBWk8k',
        '2022-09-30 08:54:53');
INSERT INTO `user_jwt`
VALUES (6981549552126005249, 1111, 'e79ed4b2-b4df-4398-8397-688804317ae1',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImU3OWVkNGIyLWI0ZGYtNDM5OC04Mzk3LTY4ODgwNDMxN2FlMSIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUzMTEyNCwiZXhwIjoxNjY1MTM1OTI0fQ.TrLUbyeYp5pnO3mr4t62EhpOh-Tq2ZP42ZMPBAQpsok',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImU3OWVkNGIyLWI0ZGYtNDM5OC04Mzk3LTY4ODgwNDMxN2FlMSIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NDUzMTEyNCwiZXhwIjoxNjY1ODI3MTI0fQ.7lyUqXbddCtSFqMyI4SCkyLwSBlehs1kDOWyhcgt2Xk',
        '2022-09-30 09:45:24');
INSERT INTO `user_jwt`
VALUES (6984358170630164481, 1111, '4bf976dc-bf09-42b5-98bb-21183c87c9ff',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjRiZjk3NmRjLWJmMDktNDJiNS05OGJiLTIxMTgzYzg3YzlmZiIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMDc1MCwiZXhwIjoxNjY1ODA1NTUwfQ.if0f-_QRdjUXxMmPWbJMW7BvmbIP-BIioSzapOXS-HM',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjRiZjk3NmRjLWJmMDktNDJiNS05OGJiLTIxMTgzYzg3YzlmZiIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMDc1MCwiZXhwIjoxNjY2NDk2NzUwfQ.kWEzH5HXWUVnIjmGW0TvPdLFUDdykqIKungSrAWLiec',
        '2022-10-08 03:45:50');
INSERT INTO `user_jwt`
VALUES (6984358202167136256, 1111, 'db1a7fae-e033-40eb-915f-9ba30587d787',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImRiMWE3ZmFlLWUwMzMtNDBlYi05MTVmLTliYTMwNTg3ZDc4NyIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMDc1OCwiZXhwIjoxNjY1ODA1NTU4fQ.6tSM86S1WiTcwVh6NECBLL2KVTPgp4Ut3aAM9fr_zgk',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImRiMWE3ZmFlLWUwMzMtNDBlYi05MTVmLTliYTMwNTg3ZDc4NyIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMDc1OCwiZXhwIjoxNjY2NDk2NzU4fQ.w6Gm5k5aqDU-44zcHY6lHo20BX-uPx_7pIY4w8Dsp9M',
        '2022-10-08 03:45:58');
INSERT INTO `user_jwt`
VALUES (6984358207015751680, 1111, '6a9727b0-74ce-4dd1-8a15-f948badfdb11',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjZhOTcyN2IwLTc0Y2UtNGRkMS04YTE1LWY5NDhiYWRmZGIxMSIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMDc1OSwiZXhwIjoxNjY1ODA1NTU5fQ._Ez8REDeZC1klIlhLMfS2TJtzE_NnzY5hNAltuYaC5o',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjZhOTcyN2IwLTc0Y2UtNGRkMS04YTE1LWY5NDhiYWRmZGIxMSIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMDc1OSwiZXhwIjoxNjY2NDk2NzU5fQ.SBdhlCnB7krIMU95_7IMYEJQoksRjD4uslFCEYeti4U',
        '2022-10-08 03:45:59');
INSERT INTO `user_jwt`
VALUES (6984358210627047424, 1111, '676ee525-d02e-4af5-a9ed-d89b0c2f557e',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjY3NmVlNTI1LWQwMmUtNGFmNS1hOWVkLWQ4OWIwYzJmNTU3ZSIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMDc2MCwiZXhwIjoxNjY1ODA1NTYwfQ.1ddKj2ct9hxN-ZdkB8AwLh-4FbmC_oPpSdhN1osEZT0',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjY3NmVlNTI1LWQwMmUtNGFmNS1hOWVkLWQ4OWIwYzJmNTU3ZSIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMDc2MCwiZXhwIjoxNjY2NDk2NzYwfQ.-ROkWlCEog_N0memee9rL4N23eMmJWBPYHPslbJruHQ',
        '2022-10-08 03:46:00');
INSERT INTO `user_jwt`
VALUES (6984358216683622400, 1111, 'eeb7a78f-dd0c-4bfc-86c4-2854a692f831',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImVlYjdhNzhmLWRkMGMtNGJmYy04NmM0LTI4NTRhNjkyZjgzMSIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMDc2MSwiZXhwIjoxNjY1ODA1NTYxfQ.H89vWIRU_bDIpsKejVKq8nAxxliRVpkYAhpDYO9tPJg',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImVlYjdhNzhmLWRkMGMtNGJmYy04NmM0LTI4NTRhNjkyZjgzMSIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMDc2MSwiZXhwIjoxNjY2NDk2NzYxfQ.R53X-WOZr4bwXk_cOoWsGjpzpz0PVxgY1FKopp9aQmA',
        '2022-10-08 03:46:01');
INSERT INTO `user_jwt`
VALUES (6984358222568230912, 1111, '2bbe5330-99d1-4d57-abaf-3be3581e2dc3',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjJiYmU1MzMwLTk5ZDEtNGQ1Ny1hYmFmLTNiZTM1ODFlMmRjMyIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMDc2MywiZXhwIjoxNjY1ODA1NTYzfQ.Wyo0cwtmHcbRaZv1kmtvPw623WWVuWAeBrDQWk2Y5Xw',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjJiYmU1MzMwLTk5ZDEtNGQ1Ny1hYmFmLTNiZTM1ODFlMmRjMyIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMDc2MywiZXhwIjoxNjY2NDk2NzYzfQ.dUgy-RGYf9EXKgL7eUpxJqusyub6_GEEMfxSXKlgnsE',
        '2022-10-08 03:46:03');
INSERT INTO `user_jwt`
VALUES (6984358226506682368, 1111, 'b00677ef-37a6-4f35-bc2d-a93fbe47b8bf',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImIwMDY3N2VmLTM3YTYtNGYzNS1iYzJkLWE5M2ZiZTQ3YjhiZiIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMDc2NCwiZXhwIjoxNjY1ODA1NTY0fQ.GPl2kl-RnX8qMsXhEv-3ZStUi8AWtQm4W-jk7wWU8k8',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImIwMDY3N2VmLTM3YTYtNGYzNS1iYzJkLWE5M2ZiZTQ3YjhiZiIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMDc2NCwiZXhwIjoxNjY2NDk2NzY0fQ.T1ZobWrQqCuHoQ2B9KQhJNQmKCqDUDAcq8j4Gxm96H0',
        '2022-10-08 03:46:04');
INSERT INTO `user_jwt`
VALUES (6984358229866319872, 1111, '70d04fe0-b47b-4c51-a732-7744c54ef911',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjcwZDA0ZmUwLWI0N2ItNGM1MS1hNzMyLTc3NDRjNTRlZjkxMSIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMDc2NSwiZXhwIjoxNjY1ODA1NTY1fQ.do7i94G5CbtsxuvSuZMLSHFRtwacsrprC79icyqj3bs',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjcwZDA0ZmUwLWI0N2ItNGM1MS1hNzMyLTc3NDRjNTRlZjkxMSIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMDc2NSwiZXhwIjoxNjY2NDk2NzY1fQ.DbpXuokKtN5oWYdRkqL8SFAMRWU8WcA69Nr9P_rsWT8',
        '2022-10-08 03:46:05');
INSERT INTO `user_jwt`
VALUES (6984358233418895360, 1111, '4a229b9d-d524-4b20-96fb-98efad0ea28e',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjRhMjI5YjlkLWQ1MjQtNGIyMC05NmZiLTk4ZWZhZDBlYTI4ZSIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMDc2NSwiZXhwIjoxNjY1ODA1NTY1fQ.qmFobnusQRCOR-HT-Q_D5ng6pyV1ONNsybV1KOOs8Po',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjRhMjI5YjlkLWQ1MjQtNGIyMC05NmZiLTk4ZWZhZDBlYTI4ZSIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMDc2NSwiZXhwIjoxNjY2NDk2NzY1fQ.79UHbhSu7OM7Hx4L30qt4GKaabSx3fP683l2bDlYQN0',
        '2022-10-08 03:46:05');
INSERT INTO `user_jwt`
VALUES (6984358238447865856, 1111, 'b300d057-c8d3-4dab-9481-99fb41373c6d',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImIzMDBkMDU3LWM4ZDMtNGRhYi05NDgxLTk5ZmI0MTM3M2M2ZCIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMDc2NywiZXhwIjoxNjY1ODA1NTY3fQ.ziArJ-Vn9PsgKfJFnqDti0O5gH5rJbpaw9-hekDbKgY',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImIzMDBkMDU3LWM4ZDMtNGRhYi05NDgxLTk5ZmI0MTM3M2M2ZCIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMDc2NywiZXhwIjoxNjY2NDk2NzY3fQ.8BuamrSxJmkmtxs8MeMHU76ASbi50QP2oyfz6q0E3Ks',
        '2022-10-08 03:46:07');
INSERT INTO `user_jwt`
VALUES (6984358241631342592, 1111, '71c85e5d-011c-41ad-9b30-aa6d19d31d19',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjcxYzg1ZTVkLTAxMWMtNDFhZC05YjMwLWFhNmQxOWQzMWQxOSIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMDc2NywiZXhwIjoxNjY1ODA1NTY3fQ.cBV6NbtZtv2Nlh1mQNYhKoA0vKw4JND-8tfU6pXGWVk',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjcxYzg1ZTVkLTAxMWMtNDFhZC05YjMwLWFhNmQxOWQzMWQxOSIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMDc2NywiZXhwIjoxNjY2NDk2NzY3fQ.rbtQwsL_yad2vSNdeM4GoaiHXDQJpCpWK2coNi1H548',
        '2022-10-08 03:46:07');
INSERT INTO `user_jwt`
VALUES (6984358423194374144, 1111, '405965d6-beec-4d87-aa5f-80abe4e67569',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjQwNTk2NWQ2LWJlZWMtNGQ4Ny1hYTVmLTgwYWJlNGU2NzU2OSIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMDgxMSwiZXhwIjoxNjY1ODA1NjExfQ.XC1wmf_MM-hdploeuDiHix9eoaFNwa2ULxS8qvdCv5A',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjQwNTk2NWQ2LWJlZWMtNGQ4Ny1hYTVmLTgwYWJlNGU2NzU2OSIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMDgxMSwiZXhwIjoxNjY2NDk2ODExfQ.uf9KtJHqmSO-4gNjkqK0jSBPUUfe0nynVogUtZp7u3g',
        '2022-10-08 03:46:51');
INSERT INTO `user_jwt`
VALUES (6984358427002802176, 1111, 'df85a5e3-7613-4ce1-9a83-69a6b83c9539',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImRmODVhNWUzLTc2MTMtNGNlMS05YTgzLTY5YTZiODNjOTUzOSIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMDgxMiwiZXhwIjoxNjY1ODA1NjEyfQ.gzq1OYCm8JzdYqGIRGchVtpzNpD9WJ2BIVP-mtCTDmY',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImRmODVhNWUzLTc2MTMtNGNlMS05YTgzLTY5YTZiODNjOTUzOSIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMDgxMiwiZXhwIjoxNjY2NDk2ODEyfQ.NkJKL6xaVjP0mWCV-e02iQrmGttomYKh50FIUKG13I8',
        '2022-10-08 03:46:52');
INSERT INTO `user_jwt`
VALUES (6984358431062888448, 1111, 'e3f171b1-5d25-425a-b12a-80dbe85a46c0',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImUzZjE3MWIxLTVkMjUtNDI1YS1iMTJhLTgwZGJlODVhNDZjMCIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMDgxMywiZXhwIjoxNjY1ODA1NjEzfQ.tX6B8Zciazrhog4LGQV7kHXddbjhjR-7IOUAAGsHeXc',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImUzZjE3MWIxLTVkMjUtNDI1YS1iMTJhLTgwZGJlODVhNDZjMCIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMDgxMywiZXhwIjoxNjY2NDk2ODEzfQ.1aucVHe4w5ahN6qAruAu80JW5jbNYaRCew2I3mMnqH8',
        '2022-10-08 03:46:53');
INSERT INTO `user_jwt`
VALUES (6984358434653212672, 1111, '3f22b226-61f8-4dda-bb1d-06983544b59b',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjNmMjJiMjI2LTYxZjgtNGRkYS1iYjFkLTA2OTgzNTQ0YjU5YiIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMDgxMywiZXhwIjoxNjY1ODA1NjEzfQ.y50HxlTFJp2OlpJcntNaGQv9RZHFLQvliql5BDUuJuI',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjNmMjJiMjI2LTYxZjgtNGRkYS1iYjFkLTA2OTgzNTQ0YjU5YiIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMDgxMywiZXhwIjoxNjY2NDk2ODEzfQ.kJHW2DeGawIbUYUX1tKRUGVVtzpMeYHMtZOqQcv36L0',
        '2022-10-08 03:46:53');
INSERT INTO `user_jwt`
VALUES (6984358440374243328, 1111, '27ef7ec2-432b-4f41-ba36-9c40ca03c258',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjI3ZWY3ZWMyLTQzMmItNGY0MS1iYTM2LTljNDBjYTAzYzI1OCIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMDgxNSwiZXhwIjoxNjY1ODA1NjE1fQ.So0avZ9W3h3mOxzc53-SjkBMFDSC8UQ0Oj5qvmdbkAk',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjI3ZWY3ZWMyLTQzMmItNGY0MS1iYTM2LTljNDBjYTAzYzI1OCIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMDgxNSwiZXhwIjoxNjY2NDk2ODE1fQ.dY_ucULg1EmAEKYNsXw__iCmHrZlyck1ukzS1NyW34A',
        '2022-10-08 03:46:55');
INSERT INTO `user_jwt`
VALUES (6984360512259756033, 1111, '2a4073da-d178-41e0-ab32-b7c072ff00ab',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjJhNDA3M2RhLWQxNzgtNDFlMC1hYjMyLWI3YzA3MmZmMDBhYiIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMTMwOSwiZXhwIjoxNjY1ODA2MTA5fQ.53wEerIDwwvqeJjFfTHmdn_kqsdJnwYvQgZ8tyU8vuM',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjJhNDA3M2RhLWQxNzgtNDFlMC1hYjMyLWI3YzA3MmZmMDBhYiIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMTMwOSwiZXhwIjoxNjY2NDk3MzA5fQ.28n5g-SD-pG1oj26Y8d4JjbLmfnLrzednPAQjqTsZe0',
        '2022-10-08 03:55:09');
INSERT INTO `user_jwt`
VALUES (6984360619164176384, 1111, 'ed7dfd16-be86-42cf-a04c-5ecfb1e1dcb0',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImVkN2RmZDE2LWJlODYtNDJjZi1hMDRjLTVlY2ZiMWUxZGNiMCIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMTMzNCwiZXhwIjoxNjY1ODA2MTM0fQ.SmssOfPL1K2PPJeajebRYe-Rb03YQV6fNha8w1OFR5Q',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImVkN2RmZDE2LWJlODYtNDJjZi1hMDRjLTVlY2ZiMWUxZGNiMCIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMTMzNCwiZXhwIjoxNjY2NDk3MzM0fQ.cnlRR30RqEIlSySfbZMu9UQeKJj6A-iE3IZuc84uMeM',
        '2022-10-08 03:55:34');
INSERT INTO `user_jwt`
VALUES (6984360835414102016, 1111, '3135d302-b3e7-438d-a902-c146ea85964b',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjMxMzVkMzAyLWIzZTctNDM4ZC1hOTAyLWMxNDZlYTg1OTY0YiIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMTM4NiwiZXhwIjoxNjY1ODA2MTg2fQ.5uAMWuNHH-di4GZOENH5xf-Xb_AXALA-y8bDVfnw8iw',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjMxMzVkMzAyLWIzZTctNDM4ZC1hOTAyLWMxNDZlYTg1OTY0YiIsInVzZXJfaWQiOjExMTEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NTIwMTM4NiwiZXhwIjoxNjY2NDk3Mzg2fQ.ntuJl2h151GbHnpyTAR5R7pUh9o_oBBo3Xcxi0oML6A',
        '2022-10-08 03:56:26');
INSERT INTO `user_jwt`
VALUES (6993449441202147329, 1, 'c5417065-5270-426f-9c29-6cc0e32a71f6',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImM1NDE3MDY1LTUyNzAtNDI2Zi05YzI5LTZjYzBlMzJhNzFmNiIsInVzZXJfaWQiOjEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NzM2ODI3OCwiZXhwIjoxNjY3OTczMDc4fQ.DA2P9kxLbIgf4Z2BNTv4Bl1Lsymt1ALNj7AeUN40jfI',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImM1NDE3MDY1LTUyNzAtNDI2Zi05YzI5LTZjYzBlMzJhNzFmNiIsInVzZXJfaWQiOjEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NzM2ODI3OCwiZXhwIjoxNjY4NjY0Mjc4fQ.Zg089g38AfqSPfAShxKMcN8QRe-ogNHvWNTiI4kpVPY',
        '2022-11-02 05:51:18');
INSERT INTO `user_jwt`
VALUES (6993449757691744256, 1, '4c8f2a57-9476-438e-9128-daa632b23026',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjRjOGYyYTU3LTk0NzYtNDM4ZS05MTI4LWRhYTYzMmIyMzAyNiIsInVzZXJfaWQiOjEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NzM2ODM1NCwiZXhwIjoxNjY3OTczMTU0fQ.s8-1SNNvxqT9bjyH019Yw2NXcuTiL-lGajYSes_CT3I',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6IjRjOGYyYTU3LTk0NzYtNDM4ZS05MTI4LWRhYTYzMmIyMzAyNiIsInVzZXJfaWQiOjEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NzM2ODM1NCwiZXhwIjoxNjY4NjY0MzU0fQ.uvu2p2q0u0zVkIzR4wYX1MKUPZ2sGsNuzX4_JqVqZpo',
        '2022-11-02 05:52:34');
INSERT INTO `user_jwt`
VALUES (6993450084667101184, 1, 'c79d1769-f00f-4c14-9293-8fca83a3f6e1',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImM3OWQxNzY5LWYwMGYtNGMxNC05MjkzLThmY2E4M2EzZjZlMSIsInVzZXJfaWQiOjEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NzM2ODQzMiwiZXhwIjoxNjY3OTczMjMyfQ.y4s0I_77PKxqVBHUbl0RxPTbBewox-RInGAD311Qs_w',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImM3OWQxNzY5LWYwMGYtNGMxNC05MjkzLThmY2E4M2EzZjZlMSIsInVzZXJfaWQiOjEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NzM2ODQzMiwiZXhwIjoxNjY4NjY0NDMyfQ.zGYKu8_izmtQu-Ry-O6-TQpqsO7G1z3yseG03cGO5qI',
        '2022-11-02 05:53:52');
INSERT INTO `user_jwt`
VALUES (6993450261347962880, 1, 'd832f58b-1773-4ce7-a54a-5f1d253cbc86',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImQ4MzJmNThiLTE3NzMtNGNlNy1hNTRhLTVmMWQyNTNjYmM4NiIsInVzZXJfaWQiOjEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NzM2ODQ3NCwiZXhwIjoxNjY3OTczMjc0fQ.Sscy2FlQQx4jY8IFkDN1KNEMxoBmWgwax4a74lS2mfI',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImQ4MzJmNThiLTE3NzMtNGNlNy1hNTRhLTVmMWQyNTNjYmM4NiIsInVzZXJfaWQiOjEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NzM2ODQ3NCwiZXhwIjoxNjY4NjY0NDc0fQ.3o_cXQcS4qLEIbt3DY1by-M4aPPCPwf1rLcxIcjgewg',
        '2022-11-02 05:54:34');
INSERT INTO `user_jwt`
VALUES (6993450476192796672, 1, 'eb0cbe23-a4b2-40fd-b6f0-5b1f61dcfbf4',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImViMGNiZTIzLWE0YjItNDBmZC1iNmYwLTViMWY2MWRjZmJmNCIsInVzZXJfaWQiOjEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NzM2ODUyNSwiZXhwIjoxNjY3OTczMzI1fQ.ztxYT9Lawr50hv8UjM38utaZjVquDNUW9HtgYG4WKYU',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImViMGNiZTIzLWE0YjItNDBmZC1iNmYwLTViMWY2MWRjZmJmNCIsInVzZXJfaWQiOjEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NzM2ODUyNSwiZXhwIjoxNjY4NjY0NTI1fQ.F5A_i26-DYKfxXxyTMDEBGvufrn2nmx5BU5CBwmEPss',
        '2022-11-02 05:55:25');
INSERT INTO `user_jwt`
VALUES (6993451346666065921, 1, 'c0a25a8f-09d3-4124-9a7e-8a83c3afb087',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImMwYTI1YThmLTA5ZDMtNDEyNC05YTdlLThhODNjM2FmYjA4NyIsInVzZXJfaWQiOjEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NzM2ODczMywiZXhwIjoxNjY3OTczNTMzfQ.6ov2TLmfI99Zz_1CyElOKgs-EAz1jj8hFWCXVP9OqtA',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ0b2tlbl9pZCI6ImMwYTI1YThmLTA5ZDMtNDEyNC05YTdlLThhODNjM2FmYjA4NyIsInVzZXJfaWQiOjEsInN1YiI6InJ1c3Qtc2hvcCIsImlhdCI6MTY2NzM2ODczMywiZXhwIjoxNjY4NjY0NzMzfQ.AzAJl3uhwjR7vX_Wk5ywvYhla1XG-wLKmi15ZHBYXMg',
        '2022-11-02 05:58:53');

-- ----------------------------
-- Table structure for user_role
-- ----------------------------
DROP TABLE IF EXISTS `user_role`;
CREATE TABLE `user_role`
(
    `user_id` bigint NOT NULL,
    `role_id` bigint NOT NULL,
    PRIMARY KEY (`user_id`, `role_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of user_role
-- ----------------------------

-- ----------------------------
-- Table structure for user_shipping_address
-- ----------------------------
DROP TABLE IF EXISTS `user_shipping_address`;
CREATE TABLE `user_shipping_address`
(
    `id`           bigint                                                        NOT NULL,
    `user_id`      bigint                                                        NOT NULL,
    `recipient`    varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL,
    `phone_number` varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci  NOT NULL,
    `address`      varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL,
    `post_code`    varchar(10) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci  NOT NULL,
    `is_default`   bit(1)                                                        NOT NULL,
    `created_time` datetime                                                      NOT NULL,
    PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of user_shipping_address
-- ----------------------------

SET
FOREIGN_KEY_CHECKS = 1;
